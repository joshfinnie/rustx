# Contributing to RustX

Thank you for your interest in contributing to RustX! We welcome contributions from the community.

## Getting Started

1. **Fork the repository** and clone it locally
2. **Set up your development environment**:
   - Install Rust 1.70+ from [rustup.rs](https://rustup.rs/)
   - Install wasm-pack: `cargo install wasm-pack`
   - Clone your fork: `git clone https://github.com/YOUR_USERNAME/rustx.git`

3. **Build the project**:
   ```bash
   cd rustx
   wasm-pack build --target web
   ```

4. **Run the examples**:
   ```bash
   cd examples/basic
   python3 server.py
   ```

## Development Workflow

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and test them thoroughly

3. Build the CDN distribution to ensure it works:
   ```bash
   ./build-cdn.sh
   ```

4. Commit your changes with a clear message:
   ```bash
   git commit -m "Add feature: description of your changes"
   ```

5. Push to your fork and submit a pull request

## Code Guidelines

- **Write idiomatic Rust** - Follow Rust conventions and style
- **Keep it simple** - RustX prioritizes simplicity over complexity
- **Add tests** - Include tests for new functionality
- **Document your code** - Add doc comments for public APIs
- **Run cargo fmt** - Format your code before committing
- **Run cargo clippy** - Fix any lints before submitting

## Testing

Before submitting a PR, make sure to:

- Run `cargo test` to ensure all tests pass
- Test your changes with the example applications
- Verify the CDN build works with `./build-cdn.sh`
- Check that the bundle size hasn't increased significantly

## Pull Request Process

1. Update the documentation if you're adding new features
2. Add examples if you're introducing new attributes or functionality
3. Ensure your PR description clearly describes the problem and solution
4. Link any related issues in your PR description
5. Be responsive to feedback and be willing to make changes

## What to Contribute

### Good First Issues

- Bug fixes
- Documentation improvements
- Adding examples
- Improving error messages
- Performance optimizations

### Feature Additions

Before working on major new features:

1. Open an issue to discuss the feature first
2. Wait for feedback from maintainers
3. Ensure it aligns with RustX's philosophy of simplicity

### Areas We'd Love Help With

- More real-world examples
- Browser compatibility testing
- Performance benchmarks
- Documentation improvements
- TypeScript type definitions
- Integration guides for popular frameworks

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors
- Follow the golden rule: treat others as you'd like to be treated

## Questions?

- Open a [discussion](https://github.com/yourusername/rustx/discussions) for questions
- Check existing [issues](https://github.com/yourusername/rustx/issues) before creating new ones
- Join our community chat (coming soon)

## License

By contributing to RustX, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to RustX!
