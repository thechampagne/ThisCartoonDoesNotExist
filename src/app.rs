/*
 * Copyright (c) 2022 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::fs;
use std::io::copy;
use rocket::http::ContentType;
use rocket::{get,catch,uri};
use rocket::response::{Content, Redirect};
use toonify::Toonify;

#[get("/")]
pub fn home() -> Content<Vec<u8>> {
    let toon = Toonify::new("https://thispersondoesnotexist.com/image", "API-Key");
    match toon.image() {
        Ok(image) => match http(&image) {
            Some(mut response) => {
                let mut vec = vec![];
                match copy(&mut response,&mut vec) {
                    Ok(_) => Content(ContentType::PNG, vec),
                    Err(_) => Content(ContentType::PNG, fs::read("image.png").expect("Something went wrong"))
                }
            },
            None => Content(ContentType::PNG, fs::read("image.png").expect("Something went wrong"))
        },
        Err(_) => Content(ContentType::PNG, fs::read("image.png").expect("Something went wrong"))
    }
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to(uri!(home))
}

fn http(image_uri: &str) -> Option<reqwest::blocking::Response> {
    match reqwest::blocking::Client::new().get(image_uri)
        .send() {
        Ok(data) => Some(data),
        Err(_) => None
    }
}