#![allow(warnings)]
#![feature(asm)]

// https://copypastatext.com/tag/morbius/

use std::arch::asm;

use rand;
use rand::distributions::{Distribution, Uniform};

// modules
use crate::metamorphic;

const HEADERS: [&str; 5] = ["My brother won’t stop saying 'ITS MORBING TIME!'",
                             "When I was 13, I came out to my parents as a Morbius male.",
                             "If Morbius has a trillion fans.",
                             "Morbius is love. Morbius is life.",
                             "Schrodinger’s Morbius."];

const PROMPT: [&str; 5] = ["Someone for the fucking life of me help my brother (22) won't stop saying 'ITS MORBING TIME!' he found these words on fb or something and said its stuck in his mind and everytime he wakes up, shits, sleeps, pisses, EVERYTHING he says 'its morbing time!'. I’m losing my sanity, fuck you morbius. That was very un-morb.",
                            "They couldnt accept my morbality, so they sent me off to camp. They just didnt understand— i was morbed that way. At the camp, they gave us electromorb therapy to morb us into beta males. I resisted the treatment, being a morbius male, you cant morb me out of being morbed. I fooled the counselors into thinking they had successfully morbed be into a beta male, and i returned home. To this day, i live a double life— one for my parents, who still cannot accept morbality, and one where its always morbin time.",
                            "I am one of them. If Morbius has 10 fans I am one of them. If Morbius has no fans, that I means I am no longer on Earth. If the Universe is against Morbius, I am against the Universe. I love Morbius until my last breath.",
                            "I was only nine years old. I loved Morbius so much, I had all the merchandise and movies. I'd pray to Morbius every night before I go to bed, thanking for the life I've been given. 'Morbius is love', I would say, “Morbius is life'. My dad hears me and calls me a faggot. I knew he was just jealous for my devotion of Morbius. I called him a cunt. He slaps me and sends me to go to sleep. I'm crying now and my face hurts. I lay in bed and it's really cold. A warmth is moving towards me. I feel something touch me. It's Morbius. I'm so happy. He whispers in my ear, “It’s morbin time'. He grabs me with his powerful morby hands, and puts me on my hands and knees. I spread my ass-cheeks for Morbius. He penetrates my butthole. It hurts so much, but I do it for Morbius. I can feel my butt tearing as my eyes start to water. I push against his force. I want to please Morbius. He roars a mighty roar, as he fills my butt with his love. My dad walks in. Morbius looks him straight in the eye, and says, 'Get morbed'. Morbius leaves through my window. Morbius is love. Morbius is life.",
                            "The line 'It's Morbin time' simultaneously exists and does not in the movie because no one has seen the film to be sure."];

pub fn print_prompt() {
    let mut rng = rand::thread_rng(); metamorphic::junk!();
    let die = Uniform::from(0..5); metamorphic::junk!();
    let throw = die.sample(&mut rng); metamorphic::junk!();

    println!("{}\n", HEADERS[throw]);
    println!("{}\n", PROMPT[throw]);
}

