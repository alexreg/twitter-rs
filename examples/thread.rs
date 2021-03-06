extern crate egg_mode;

mod common;

use std::collections::{VecDeque, HashSet};
use egg_mode::tweet;

fn main() {
    let c = common::Config::load();

    //Thread Reconstruction
    //
    //This is fairly imperfect, but still effective enough for use in a regular client or with
    //recent-enough tweets. The idea is that we have an arbitrary tweet and we want to find tweets
    //that it replied to, and tweets the user posted that reply to it.
    //
    //The first part is fairly easy: it's essentially a linked list, where each node traversal is a
    //call to tweet::show. Since we're receiving these tweets in reverse-chronological order,
    //calling push_front makes sure the order of the eventual thread is properly chronological.
    //
    //The latter part calls for some creative use of tweet::user_timeline. First we set up the
    //timeline itself, indicating that replies should be included in the response. Then, rather
    //than calling start() to get the most recent, we use call() directly, saying "give me tweets
    //posted since this ID". Since this could include tweets that aren't on the same thread, we
    //need to make keep track of the tweet IDs in the thread, so we can make sure the tweet itself
    //is a reply to something within the thread. Since the timeline returned by this call is in
    //reverse-chronologocal order, we rev() the output page to make sure we're pushing tweets in
    //chronological order. If we wanted to fill our thread buffer completely, we could keep calling
    //newer() for a certain number of pages or until we've hit our cap, but using the default 20 is
    //sufficient for this demo.

    //The example post used in this demo is the fourth post in a seven-post thread I
    //(@QuietMisdreavus) posted shortly before writing this. You can easily extrapolate this into a
    //function that takes i64 as needed.
    let start_id: i64 = 773236818921873409;

    println!("Let's reconstruct a tweet thread!");

    let mut thread = VecDeque::with_capacity(21);
    let mut thread_ids = HashSet::new();

    let start_tweet = tweet::show(start_id, &c.con_token, &c.access_token).unwrap();
    let thread_user = start_tweet.response.user.id;
    thread_ids.insert(start_tweet.response.id);
    thread.push_front(start_tweet.response);

    for _ in 0..10 {
        if let Some(id) = thread.front().and_then(|t| t.in_reply_to_status_id) {
            let parent = tweet::show(id, &c.con_token, &c.access_token).unwrap();
            thread_ids.insert(parent.response.id);
            thread.push_front(parent.response);
        }
        else {
            break;
        }
    }

    let replies = tweet::user_timeline(thread_user, true, false, &c.con_token, &c.access_token);

    for tweet in replies.call(Some(start_id), None).unwrap().into_iter().rev() {
        if let Some(reply_id) = tweet.response.in_reply_to_status_id {
            if thread_ids.contains(&reply_id) {
                thread_ids.insert(tweet.response.id);
                thread.push_back(tweet.response);
            }
        }

        if thread.len() == thread.capacity() {
            break;
        }
    }

    for tweet in &thread {
        println!("");
        if tweet.id == start_id {
            println!("-- this is our starting tweet");
        }
        common::print_tweet(&tweet);
    }
}
