# cs128honorsfinalproj

Group Name: We're Lovin It

Group Members: Ryan To (rto4), Cynthia Di (cdi5), Priyanka Jakka (pjakka3)

Inspiration: The Stephen Patula Youtube Channel (https://www.youtube.com/c/StephenPatula)

Project Introduction:
We'll be making a McDonald's assembly line with custom graphics. Basically, imagine a top view of a McDonald's. it would have a burger grilling station, a drink station, a fry station, an order assembly station, and an order station. We would have the user input their order, and then see how that order would be assembled. Every station would be running in parallel, depending on the requirements of the order. Each of those would run on their own threads, showing their progress, and would complete the order. We could stack multiple orders, and that would show the true capability of a busy McDonald's. We were inspired by the YouTube channel above, where he does POV videos of him working at McDoanld's. Since he's showed the POV of many different stations, such as assembly, grilling, drinks, fries, ect, and we know all of that happen simultanously in a normal McDoanld's the idea came of using threads to emulate that work. 

Technical Overview:
The user interface will be a 2D top down view of a McDonald's kitchen. It will contain a basic map of the inside of the kitchen, and will have workers at each station ready to prepare orders. This will be sort of like a game, where the user will input an order, and will see the order being constructed, and the progress of each individual thread as it executes. The user input will be done via sets of menus, where the user can choose what they want to order, and add it to a list. After they have added what they want, they will place the order, and then see all the individual parts moving. We'll have progress bars on top of each station to indicate the progress.  Once completed, the user will be "presented" with the order, and the kitchen shall remain idle, unless another order was placed before the first completed. 

Checkpoint 1:
A basic graphics should be implimented by this point, with the kitchen laid out and workers in place. The menu ordering should also be implimented, although without the thread processing yet. 

Checkpoint 2:
Parallel thread processing for all the parts of the kitchen when placing an order should be implimented. 

Challenges:
There will be a lot of challenges to this, since we've never used graphics crates before in Rust, and we're going very deep into using parallel threads to process things. The menu should give us the leeway needed to overcome these challenges, as we can make it as complex or as simple as we want to, depending on our struggles with implimenting the rest. 
