# Image Converter

## A Terminal tool to convert images from one format to another and also remove backround

This is a simple solution to convert images from one format to another and also remove backround from images. It has a lot of features:

* Supports conversion between 3 major formats jpg, png and webp.

* A one command run feature with args.

* A nice terminal interactive ui if u want that.

* Backround removal using a very nice model.

* Shows you a nice color palette from your image.

* Strips all metadata if you want.

* It is useful for web developers who constantly need this conversion.

* It has good logs and error handling.

## How to run locally

1. Clone the repository

``` 
git clone https://github.com/Turbash/image_converter.git
cd image_converter
 ```

2. Build the project

```
cargo run
```

or

1. Download the execuable for ur OS from releases and extract it

2. Open the terminal and navigate to the extracted image_converter folder

3.Run it simply like any other executable.

```
./image_converter
```

## Usage instructions

1. For interactive method, no real instructions required here, everything is there in ui, just run the executable or if running from source code, use cargo run.

2. For one line command, there are few args u can use:

``` 
cargo --help
```

or if using executable

```
./image_converter --help
```

3.The args are clearly explained here in help. This allows one line usage.

## Requirements 

* Rust (if running from source code) https://www.rust-lang.org/tools/install

* ONNX Runtime library (libonnxruntime.so for Linux, onnxruntime.dll for Windows). U only need these if u are running release build from source code. if running dev build from source code cargo handles (in linux atleast I use linux). And in release versions these are already added.

*U2-Net ONNX model (u2net.onnx) in the models directory

## Model

The project uses U2-Net model for backround removal.

## License

This project is licensed under the MIT License.
You are free to use, modify, and distribute this software for any purpose.

## Contributing Guide

Feel free to fork the project and play with the code. If you want to conribute to the project, please open an issue first. And then do a pull request, I will then verify ur request and if it is good i will accept it.