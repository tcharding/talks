Intro to Kernel Hacking - finding things to work on
===================================================

So you want to be a kernel hacker but you don't know where to start ...

Motivation
----------

Some reasons why you might _not_ want to do kernel development:
 - It's hard.
 - Some kernel developers are _prickly_.
 - It's not _cool_, we still use email.
 - It can be lonely, if you get stuck working on something that nobody cares
   about. 

So, why do you want to do kernel development?
 - What motivates you to get up in the morning and stand/sit in front of your
   keyboard?
 - How much time have you got to devote to this? It could easily be the best
   part of a year (full time) before you are _really_ doing anything useful.

Some reasons you might _want_ to do kernel development:
 - It's _cool_, you can use email.
 - If you are the sort of person who likes programming, systems programming,
   systems programming in C, open source systems programming in C ...
 - Most kernel developers are polite, well mannered, and extremely generous
   with their time. Many will go out of their way to help you, if you ask the
   correct questions in the correct manner and are seen to be putting in
   effort.

Basically, if this sort of work is meaningful to you on an individual level and
you feel you have the mental, financial, and temporal resources then you should
definitely pursue it - for the right person kernel hacking is a _seriously_
satisfying way to pass your days.


Skills
------

'The main issues faced by any large software system are not technical issues but
people issues'

Get some people skills, read some books:
 - How to win friends and influence people, better named: 'how not to be a dick'.
 - Pay attention, have 'beginners mind'. Everything is done in the open so
   observe how others act and imitate.
 - Aim to be a zero

    https://opensource.com/article/17/8/open-source-success-starts-zero

### Technical

Learn C _well_:
 - The C Programming Language - Brian W. Kernighan and Dennis M. Ritchie.
 - The Linux Programming Interface - Michael Kerrisk.

Unless you have written _a lot_ of C then do yourself a favour and complete
these books including all exercises.

Learn OS theory (and Linux in particular):

 - Operating System Concepts - Avi Silberschatz, Peter Baer Galvin, Greg Gagne. 
 - Linux Device Drivers - Jonathan Corbet, Alessandro Rubini, Greg Kroah-Hartman.
 - Linux Kernel Development - Robert Love.


Finding Things to Work On
-------------------------

Heuristics:
 - Work on what interests you.
 - Focus on interesting _tractable_ problems.

One method of progression:
 - Do your first patch.
 - Do your first patch set.
 - Do a bunch of checkpatch cleanup patches in staging.
 - Get some real hardware and write a driver
   - Pick hardware with a driver currently in staging.
   - Pick hardware that has a similar driver in tree.

then ...

### Moving on from Staging

This talk/document hinges on two points:
 1. You can't/shouldn't do cleanups outside of staging.
 2. You can't change code when you don't understand it.

And one kicker:
 1. Even when you find something that needs doing, most likely it will not get
    accepted. Or not easily.

This is because the kernel is based on trust. You get patches in the kernel when
people trust that you will be around to fix the bugs you introduce - yes _you_
will introduce bugs. Hence, the ramblings about motivation a priori.

However:
 - Kernel hackers tend to have more things to work on than they have time.
 - You can convince such a hacker to give you some of their work.

But hang on!

Giving work to newbies takes effort too
 - It takes time to explain it.
 - They have a nasty habit of pestering you when they either can't do it or
   they do it wrong.

Get some people skills! You can convince hackers to spend some of their time
on you, they benefit because:
 - It's nice when things that you want done get done by other people.
 - It's nice when people respect you, asking someones guidance is a sign of respect.
 - You learn things by teaching, hackers tend to like learning things.

You need to convince them, and yourself, that you are worth the effort because:
 - You have the base skill set (see above).
 - You are putting in the effort.
 - You pay attention, no one likes saying things twice.
 - You are pleasant to interact with (you heard the bit about people skills
   right?). 

You don't always need to ask directly. Often while talking, kernel developers
may mention some thing they have been meaning to do, or they wish someone would
do. Make a note of it, have a go at it, if you can get part way started that is
enough to show 3 of the 4 points above. They will most likely step in and guide
you on the things you are stuck on.

Talk to real kernel developers
 - Go to conferences.
 - Lurk on the mailing lists (kernel newbies and driver dev list).

If anyone mentions anything that feel you can do or you are interested in
researching then do it. This is a gift economy, try to give without taking. Do
stuff that helps others while demanding as little as possible of them in return.


Summing Up
----------

Finding things to work on in the kernel may be not so much about the technical
aspects of finding something to work on but rather the social aspects.

Be patient, while at first it seems impossible to find something to do, very
soon you will be swamped in tasks. There is unlimited amounts of work to do and
no rush, take your time, do what you do meticulously.

Expect that your changes will not get in, or if they do, not for a good
while. You may have more success if you separate your patches from your ego. Or
follow the concept of working not for the fruit of your labour but for the
labour itself.

Finally, when you wake up one morning and a patch set you put in has 20 response
to it or the first time you get an email from <insert kernel god here>, it will
all be worth it. Money can't buy that feeling.

Some of the best programmers in the world work on the kernel, and a bunch of
them are very entertaining to work with.