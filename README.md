# Basic STRIPS Planner
The following is a very basic **S**tanford **R**esearch **I**nstitute **P**roblem **S**olver, implemented in Rust. I've left the following is my 3rd implementation after having studied more sophisticated implementations such as [GameReadyGoap](https://github.com/Joy-less/GameReadyGoap/) and [godot-goap](https://github.com/viniciusgerevini/godot-goap). While I did program a similar A* search algorithm to GameReadyGoap, I thought the code was too similar to the point of plagerism, and so this 3rd implementation utilizes everything I have learned from repeatedly programming this specific project, and making it my own. 
# How to Use
I've left a very basic program in `main.rs`, which should be able to compile and run as is when this repository is cloned. The 
# What's next?
I'm honestly very proud of this very simple STRIPS planner, but I know it can be improved beyond its current state, for example, it cannot handle multiple conditions in the goal. This is in large part because I did not want to implement a partial order planner. In the future, in some alternative repository, I would like to look into a heuristic based solver.
# References
1. [How to Model Classical Planning Problems with STRIPS](https://www.youtube.com/watch?v=uFPx6yoXz1k)
2. [GameReadyGoap](https://github.com/Joy-less/GameReadyGoap/)
3. [godot-goap](https://github.com/viniciusgerevini/godot-goap)