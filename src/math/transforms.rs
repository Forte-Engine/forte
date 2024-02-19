use cgmath::*;

/// A representation of a transform using a vector3 position, and scale, and a quaternion rotation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// The position of the transform.
    pub position: cgmath::Vector3<f32>,

    /// The rotation of the transform.
    pub rotation: cgmath::Quaternion<f32>,

    /// The scale of the transform.
    pub scale: cgmath::Vector3<f32>
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0, 0.0).into(),
            rotation: (0.0, 0.0, 0.0, 1.0).into(),
            scale: (1.0, 1.0, 1.0).into()
        }
    }
}

impl Transform {
    /// Convert the transform to a 4x4 matrix using translation * rotation * scale matrices.
    pub fn to_mat(&self) -> Matrix4<f32> {
        let scale = cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation = cgmath::Matrix4::from(self.rotation);
        let translation = cgmath::Matrix4::from_translation(self.position);
        return translation * rotation * scale;
    }
}
