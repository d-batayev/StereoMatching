# Implementing a Stereo Matching Algorithm
## Table of Contens
* ### What is Stereo Matching and Why Do We Need It?
* ### Algorithms Overview 
* ### Requirements

## What is Stereo Matching and Why Do We Need It?

When our eyes view a scene, the left and right eyes see slightly different things. The left and right images are known as a stereo pair. By matching the corresponding points in the two images, we are able to infer the depth of the scene.

## Algorithm Overview
The attached paper http://www.bmva.org/bmvc/1992/bmvc-92-035.pdf proposes using a dynamic programming algorithm to perform stereo matching. Note the following points:
* The algorithm is only focused on grayscale images (no alpha channel and also only one scalar value by pixel intensity). Therefore, vectors and matrices collapse to scalar values in that specific case (specifically, the covariance matrix becomes the variance).
* As described in the paper, we first do the forward pass and build the cost matrix of matching pixels in each row. One pixel can only match to at most one pixel in the same row in the other image. There are three cases for matching:
 * Sub Two pixels do match
 * Sub The first pixel is occluded and the second pixel is unmached
 * Sub The second pixel is occluded and the first pixel is unmached

To infer the depth of the image, we then implement the backward pass. Starting from i=M and j=N (and until i=0 and j=0), you will move along one of the three arrows starting from the current point:
* going up: if pixel i is unmatched;
* going left: if pixel j is unmatched;
* going upper left: if pixels i and j match;

The distance between pixel i and j (if they match) is linked to the depth and is called disparity. As an output of the algorithm, you should display the disparity map (showing the disparity for each pixel). The output of the algorithm is one disparity map generated from left and right views of the same scene.

The Random Dot Stereogram was also generated to better explain and understand the algorithm. (See Explanation directory)

# Requirements

The Algorithm was implemented in the Rust Language for learning purposes and because it is a compiled language. While Rust generates a disparity map for a pair of images in less than a second, Python3 takes more than 10 minutes for the samepair of images. Rust has a very convenient image processing library (image), which was used to manipulate the images. Rand library was also used to generate the random dot stereograms. Therefore, to run the program, add the following dependencies to your Cargo.toml:

``` bash
image = "0.23.0"
rand = "0.7.0"
```
 


Daulet Batayev - March 2020
