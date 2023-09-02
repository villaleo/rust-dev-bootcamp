#![allow(dead_code)]

// The compiler follows three rules to determine the lifetime of a return value. If the compiler
// still cannot infer the output lifetime paramter after applying each rule, you will need to
// explicitly use lifetime annotations.
//
// Here are the rules:
// 1. Each reference parameter gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output
//    parameters.
// 3. [Only applies to methods] If there are multiple input lifetime parameters, but one of
//    them is &self or &mut self, the lifetime of self is assigned to all output lifetime
//    parameters.

// When storing references in structs, we must use generic lifetime annotations.
struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    // Lifetime elision lets us omit the lifetime annotation on the return type because of rule #3.
    fn edit(&mut self, new_content: &'a str) -> &str {
        let old_content = self.content;
        self.content = new_content;
        old_content
    }
}

// Lifetime elision lets us omit lifetime annotations altogether because of rule #2.
fn take_and_return_content(content: &str) -> &str {
    content
}

fn main() {
    let mut tweet = Tweet { content: "Hello!" };
    let unedited_tweet = tweet.edit("Hey..");
    println!("User @ 00:00\n  {}", unedited_tweet);
    println!("User @ 01:32 (edited)\n  {}", tweet.content)
}
