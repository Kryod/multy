# Multy

An API in rust that does a lot of stuff.

## Algorithm

### Filters and `parameters`

- blur - `radius`
- dilate - `radius`
- erode - `radius`
- local contrast - `radius`, `factor`
- median blur - `radius`
- min max - `radius`
- *and much more in the future ...*

### ML

Work in progress. no available features yet.

## Routes

### [POST] `/apply`

Apply a specific algorithm on a provided image, then return the processed image.

#### Parameters

  - `algorithm`: among [filters](#filters-and-parameters), simply replace space by underscore
  - `radius`: provide the radius who should be used for the selected algorithm
  - `factor`: if needed<sup>[1](#help)</sup>, provide the factor for the specified algorithm. Otherwise, this parameter will be ignored
  - `photo`: file field containing the target image

#### Return

On success, status code 200, also known as `OK`, with the processed image in the body. Otherwise return status code 400, `BAD REQUEST`, with the error message in the body.

### [GET] `/public`

Allow user to select and see an image<sup>[2](#help)</sup> stored on the server.

### [GET] `/public/<file_name>`

Load and display the requested `file_name`. Return status code 404, `NOT FOUND`, if the file doesn't exist.

### [POST] `/save`

Store the `photo` on the server, without any other traitements.

#### Parameters

  - `photo`: file field containing the image to store

#### Return

On success, status code 201, also known as `CREATED`, with the path of the saved image on the header `location`. Otherwise return status code 400, `BAD REQUEST`, with the error message in the body.

## Help

1. See [filters](#filters-and-parameters) section to known which parameter is needed for selected algorithm
1. Can be any image previously send or processed by the server.
