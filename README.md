# models-cv
Crate for loading gltf/obj files and projecting the verticies using camera models. Used to crate ground truth features for SFM pipelines. 

See save_points examples on how the camera/trajectories are defined.

### Coordiante System
Currently the views/feautres are defined in the Computer Graphics coordinate system of +X Right, +Y Up, +Z Back

For a Computer Vision coordiante system roate poses around +X with a value of PI. For features invert the y component.

### Sphere
![sphere](doc/camera_features_sphere_1.png)
![sphere](doc/camera_features_sphere_2.png)
![sphere](doc/camera_features_sphere_3.png)


### Suzanne
![suzanne](doc/camera_features_Suzanne_1.png)
![suzanne](doc/camera_features_Suzanne_2.png)
![suzanne](doc/camera_features_Suzanne_3.png)
![suzanne](doc/camera_features_Suzanne_4.png)
![suzanne](doc/camera_features_Suzanne_5.png)
![suzanne](doc/camera_features_Suzanne_6.png)
![suzanne](doc/camera_features_Suzanne_7.png)

### Assets

GLTF: https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0 (Suzanne)
OBJ: https://github.com/odedstein/meshes (Sphere)
