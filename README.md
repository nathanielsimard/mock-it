# Mock It
This library aims to make mocking reliable.
Most mocking libraries in Rust are experimental using code generation with rust nightly.
It is great in term of ease of use, but it has limitations.
It is not the most robust way of mocking yet.
This is where `Mock_it` tries to fill the gap. 
You will have to implement the trait you are willing to mock, but without coding any logic.
This way you can be sure your mock works as expected without having to maintain it much.