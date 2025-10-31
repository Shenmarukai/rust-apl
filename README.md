# Rust-APL
- Experimental APL interpreter in Rust. LGPL 3.0 License.
- Aiming for a feature complete APL2 interpreter *and maybe compiler?*.

***Respect to the original creator/developer [AngryLawyer](https://github.com/AngryLawyer) for making the initial implementation of this 13 years ago!***

## Initial Goals:
- Convert to modern Rust.
- APL2 feature parity.
- GPU acceleration via [Rust-GPU](https://github.com/Rust-GPU/rust-gpu)

## Future Goals:
- Possibly extend APL2 with *unit tagged* numbers:
  - This would enable unique operator behavior between like and different *unit tagged* numbers.
  - This would enable a form of type safety without introducing the complexity of *composite / aggregate* types.
  - This could enable the generalization / extension of complex numbers into the realm of quaternions or the multivectors of geometric algebras.
- Possibly banch APL into a new compiled language.
  - This would enable apl to operate in more resource constrained environments like micro-controllers.
  - This could enable zero cost *unit tagged* operator specialization.
  - This could enable making APL a performant inline language like [rust-cpp](https://github.com/mystor/rust-cpp) does with C++.
 
## Personal Motivations
I have been working on a large neural-networks and robotics library in rust *originally in c++* under the organization [AtomuranRobotics](https://github.com/AtomuranRobotics), and I realized I spent so much time trying to decide *how* to implement the mathematics
instead of just *implementing* the darn mathematics. I yearned to be able to just write the mathematics in a consise way but the implementations to do so would wrap right back around to the original problem.
Once I learned about APL I realized it was exactly the syntax I was looking for, consise, explicit, and generalized. There were two problems though, one: the lanuage is interpreted, which is not great for embedded systems in robotics,
and two: the language treats all numbers the same, which for something like geometric algebra that is littered with explicit unit vectors/blades that require specialized operator behavior depending on whether like or dislike unit vectors are multiplied,
is a problem. This could be handled in normal APL, but it would not be elegant. Because units are so integral to mathematics, and with APL being a mathematically inspired language, I believe it would benefit from their inclusion.

***Definitely not ready for actual use. You have been warned.***
