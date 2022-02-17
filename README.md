# Silent Learner

<img src ="https://user-images.githubusercontent.com/58668163/154505075-73007101-bfae-408b-8f2e-e88761c7c836.png"/>
Convenient command line tool for learning, based on spaced repetition technique.
Currently in very early stage of development.

### Features
Implemented features:
- flashcard creation and review
- simple, colorful cli interface
- spaced repetition based review algorithm

Planned features:
- installation script
- manual page
- binary packages for Linux/Unix systems
- more detailed README.md
- labels for flashcards
- more flexible review algorithm
- config file for better customization
- multiplatform support (for now should be working on Linux systems)

### Prerequisites

To install from source, following are required:
- git
- rust 1.56.0 or higher (2021 edition) https://www.rust-lang.org/tools/install

### Installing

Run following commands:
```
git clone https://github.com/JayJaySea/silent-learner
cd silent-learner
cargo build --release
```

Then, create symlink to executable in any PATH directory:
```
ln -s <full_path_to_PATH_dir> <full_path_to_cloned_repo>/target/release/card
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
