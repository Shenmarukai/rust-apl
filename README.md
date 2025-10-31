# Rust-APL

### Experimental APL interpreter in Rust. LGPL 3.0 License.

***Respect to the original creator/developer [AngryLawyer](https://github.com/AngryLawyer) for making the initial implementation of this 13 years ago!***

### Aiming for a feature complete APL2 interpreter *and maybe compiler?*.

## Initial Goals:
- Convert to modern Rust.
- APL2 feature parity.
- GPU acceleration via [Rust-GPU](https://github.com/Rust-GPU/rust-gpu)

## Future Goals:
- Possibly extend APL2 with *unit tagged* numbers:
  - This would enable a form of type safety without introducing the complexity of *composite / aggregate* types.
  - This would enable unique operations behavior between like and different *unit tagged* numbers.
  - This could enable the generalization / extension of complex numbers into the realm of quaternions or the multivectors of geometric algebras.
- Possibly banch APL into a new compiled language.
  - This would enable apl to operate in more resource constrained environments like micro-controllers.
  - This could enable zero cost *unit tagged* operator specialization.
  - This could enable making APL a performant inline language like [rust-cpp](https://github.com/mystor/rust-cpp) does with C++.

***Definitely not ready for actual use. You have been warned.***
