/* the intial goal is that i am gonna write out all the equations by hand in here first, then manually implement the derivation in code
to fully appreciate the logistics and mechanics of the error state kalman filter system or more specfically in this case the multiplcative error state extended kalman filter on SO(3)
so the order of how rotations work with SO(3), so(3) and rodrigues rotation formula:
unit quartenion -> log map pulls it down to 3d vector in so(3) -> exponetial map then to turn it back to a unit quartenion (rodrigues formula)



eskf full run through:

predict step:

integerate nominal state measurment forward to current time

integerate nominal state covariance forward to curren time.

regardless of measurment noise only taking into account process noise for the covariance

measurment step:

comput innovation / residue (difference between what the prediction says for the measurment and what the sensor says)

compute innovation covariance

kalman gain (eg recombination of how much to weigh the prediction vs the sensor measurment readings more weight means more trust and more share of what actually gets injected.)

computer error state estimate eg whats gets injected

update or propogate covariance according to the kalman gain 

inject the error state error back into the nominal state

reset the error state matrix -> 0

update covariance readjusting for the new nominal state, eg apply a small correction equivalent to new error state.


https://arxiv.org/pdf/1711.02508 - solas paper for the actual derivation i am using.

https://arxiv.org/pdf/1812.01537v9 - for a bit more flavourful theory.

*/

use nalgebra::*;
use libm::{powf, sqrt};
use libm::{cos, sin};
const NOM: usize = 13;
type NominalState = SVector<f32, NOM>;

const ERROR_S: usize = 12;

type ErrorState = SVector<f32, ERROR_S>;
type ErrorCov = SMatrix<f32, ERROR_S, ERROR_S>;


const Procces_Noise: usize = 12;

type Process_Noise = SMatrix<f32, Procces_Noise, Procces_Noise>;


type ImuMeasVec = SVector<f32,3>;


struct ESKF {
    nominal_state : NominalState,

    error_state : ErrorState,

    error_state_covariance : ErrorCov, 
}



// TODO -> decide what to have gps noise as? idk yet


struct Noise {
    process_noise : Process_Noise,

}

type Rotation_vector = SVector<f32, 3>;
type Rotation_matrix = SMatrix<f32, 3,3>;
type quartenion_vec = SVector<f32,4>;
/*
  thinking of the matrix being:
  velocity 3x3
  rotation (quartenions) 4x4
  accel bias 3x3
  gyro bias 3x3
*/

impl ESKF { // honestly TODO -> perhaps experiment with some derive macros for intializing the ESKF struct and also perhaps the monte carlo harness aswell?
    fn new() -> Self {
    Self {
    nominal_state: {
        let mut variable_nom = NominalState::zeros();
        variable_nom[3] = 1.0;
        variable_nom

    },
    error_state : {
       let mut g = ErrorState::zeros();
       g
    },
    error_state_covariance: {
        let mut d = ErrorCov::zeros();
        d
    },


    }

    } // 0,1,2 vel, 3,4,5,6, quart (3 is the scalar quantity), 7,8,9 accel b, 10,11,12 rotation b
    fn predict (&mut self, imu: ImuMeasVec, dt : f32, noise : Noise) {
    

      let mut rotation_m = Rotation_matrix::zeros();
      rotation_m[(0,0)] = self.nominal_state[3] * self.nominal_state[3] + self.nominal_state[4]  * self.nominal_state[4]  - self.nominal_state[5] * self.nominal_state[5] - self.nominal_state[6] * self.nominal_state[6];
      rotation_m[(0,1)] = 2.0 * (self.nominal_state[4] * self.nominal_state[5] - self.nominal_state[3] * self.nominal_state[6]);
      rotation_m[(0,2)] = 2.0 * (self.nominal_state[4] * self.nominal_state[6] + self.nominal_state[3] * self.nominal_state[5]);
      rotation_m[(1,0)] = 2.0 * (self.nominal_state[4] * self.nominal_state[5] + self.nominal_state[3] * self.nominal_state[6]);
      rotation_m[(1,1)] = self.nominal_state[3] * self.nominal_state[3] + self.nominal_state[4] * self.nominal_state[4] - self.nominal_state[5] * self.nominal_state[5] - self.nominal_state[6] * self.nominal_state[6];
      rotation_m[(1,2)] = 2.0 * (self.nominal_state[5] * self.nominal_state[6] - self.nominal_state[3] * self.nominal_state[4]);
      rotation_m[(2,0)] = 2.0 * (self.nominal_state[4] * self.nominal_state[6] - self.nominal_state[3] * self.nominal_state[5]);
      rotation_m[(2,1)] = 2.0 * (self.nominal_state[5] * self.nominal_state[6] + self.nominal_state[3] * self.nominal_state[4]);
      rotation_m[(2,2)] = self.nominal_state[3] * self.nominal_state[3] + self.nominal_state[4] * self.nominal_state[4] - self.nominal_state[5] * self.nominal_state[5] - self.nominal_state[6] * self.nominal_state[6];
     // constructing the quartenion rotation to rotate acceleration from body -> world frame for velocity integeration.
     // 3,4,5,6 = w,x,y,z
 // see 2.5 for the quartenion rotation matrix equation used.

     let mut true_ax = (imu[0] - self.nominal_state[7]);
     let mut true_ay = (imu[1] - self.nominal_state[8]);
      let mut true_az = (imu[2] - self.nominal_state[9]);
     let mut accel_RVec = Rotation_vector::zeros();
     accel_RVec[0] = true_ax;
     accel_RVec[1] = true_ay;
     accel_RVec[2] = true_az;
   
    let global_matrix = rotation_m * accel_RVec;
    
    let mut gravity_vector = Rotation_vector::zeros();
    gravity_vector[2] = -9.81; // for now this stays but means i have to tinker with some stuff later.
    
    let mut v_change = (global_matrix + gravity_vector) * dt;
    self.nominal_state[0] += v_change[0];
      self.nominal_state[1] += v_change[1];
       self.nominal_state[2] += v_change[2];
     // pretty self explantory this is just integerating velocity forward, taking our direction to be up or ENU


       let mut true_rotation_x = (imu[3] - self.nominal_state[10]) * dt; // rotation vector x , y , z
      let mut true_rotation_y = (imu[4] - self.nominal_state[11]) * dt;
       let mut true_rotation_z = (imu[5] - self.nominal_state[12]) * dt;
    
    // w & dt = rotation vector or lowercase phi.
  
    // pythagorous essentially calculating the magnitude of a condesened rotation representing all 3 culmulatively.
   let angle = sqrt((true_rotation_x * true_rotation_x + true_rotation_y * true_rotation_y + true_rotation_z * true_rotation_z) as f64) as f32;
     
     let mut quartenion_vec = quartenion_vec::zeros();

    if angle < 1e-6 {
    let dq0: f32 = 1.0;
    let dq1: f32 = true_rotation_x * 0.5;
    let dq2: f32 = true_rotation_y * 0.5;
    let dq3: f32 = true_rotation_z * 0.5;
    quartenion_vec[0] = dq0;
    quartenion_vec[1] = dq1;
    quartenion_vec[2] = dq2;
    quartenion_vec[3] = dq3;
   } else {
    let dq0: f32 = cos((angle / 2.0) as f64) as f32;
    let dq1: f32 = sin((angle / 2.0) as f64) as f32 / angle;
    let dq2: f32 = sin((angle / 2.0) as f64) as f32 / angle;
    let dq3: f32 = sin((angle / 2.0) as f64) as f32 / angle;
    quartenion_vec[0] = dq0;
    quartenion_vec[1] = dq1;
    quartenion_vec[2] = dq2;
    quartenion_vec[3] = dq3;
   } // little bit clunky but it works.

    // the / angle if for the unit length this is essentially just cos pheta / 2 + u sin (pheta / 2)
   // to calculate the unit length you need to do phi or the rotation vector / the magnitude of that rotation vector which equals angle in this case.
   // however i need to do it in this way because each component x y and z CONTRIBUTE to the vector and therefore you need to do it seperately so you get a VECTOR which is your unit "direction"
   // rather than jsut a scalar value which could be in any direction which isnt helpful.
  

  // 1.2.2 equation from sola paper or the product equation p x dp
  let new_q0 = self.nominal_state[3] * quartenion_vec[0] - self.nominal_state[4] * quartenion_vec[1] - self.nominal_state[5] * quartenion_vec[2] - self.nominal_state[6] * quartenion_vec[3];
  let new_q1 = self.nominal_state[3] *  quartenion_vec[1] + self.nominal_state[4] * quartenion_vec[0] + self.nominal_state[5] * quartenion_vec[3] - self.nominal_state[6] * quartenion_vec[2];
  let new_q2 = self.nominal_state[3] * quartenion_vec[2] + self.nominal_state[4] * quartenion_vec[3] + self.nominal_state[5] * quartenion_vec[0] + self.nominal_state[6] * quartenion_vec[1];
  let new_q3 = self.nominal_state[3] * quartenion_vec[3] + self.nominal_state[4] * quartenion_vec[2] - self.nominal_state[5] * quartenion_vec[1] + self.nominal_state[6] * quartenion_vec[0];

  //  shouldnt need to renormalise because f32 rounding points are tiny and neglible in practice.
  
  self.nominal_state[3] = new_q0;
  self.nominal_state[4] = new_q1;
   self.nominal_state[5] = new_q2;
    self.nominal_state[6] = new_q3;


   let mut fx = ErrorCov::zeros();
   // change in v effect on change in v
   fx[(0,0)] = 1.0; fx[(1,1)] = 1.0; fx[(2,2)] = 1.0;
   
   // change in rotations effect on change in v
   

    // building the skew symmetrix matrix based on 2.3.1
    
    let mut skew_symmetric_m = Rotation_matrix::zeros();
    skew_symmetric_m[(0,1)] = -1.0 * true_az;
     skew_symmetric_m[(0,2)] = 1.0 * true_ay;
     skew_symmetric_m[(1,0)] = 1.0 * true_az;
      skew_symmetric_m[(1,2)] = -1.0 * true_ax;
      skew_symmetric_m[(2,0)] = -1.0 * true_ay;
       skew_symmetric_m[(2,1)] = 1.0 * true_ax; 

       // 270 fx change in rotation effect on change in velocity.
    let Dv_rotation = -rotation_m * skew_symmetric_m * dt;
    fx[(0,3)] =  Dv_rotation[(0,0)]; fx[(0,4)] =  Dv_rotation[(0,1)]; fx[(0,5)] =  Dv_rotation[(0,2)];
    fx[(1,3)] =  Dv_rotation[(1,0)]; fx[(1,4)] =  Dv_rotation[(1,1)]; fx[(1,5)] =  Dv_rotation[(1,2)];
    fx[(2,3)] =  Dv_rotation[(2,0)]; fx[(2,4)] =  Dv_rotation[(2,1)]; fx[(2,5)] =  Dv_rotation[(2,2)];
    
    // velocity effect on acceleration bias
    let dv_accelb = -rotation_m * dt;
    fx[(0,6)] = dv_accelb[(0,0)];  fx[(0,7)] = dv_accelb[(0,1)]; fx[(0,8)] = dv_accelb[(0,2)];
     fx[(1,6)] = dv_accelb[(1,0)];  fx[(1,7)] = dv_accelb[(1,1)];  fx[(1,8)] = dv_accelb[(1,2)];
      fx[(2,6)] = dv_accelb[(2,0)];  fx[(2,7)] = dv_accelb[(2,1)];  fx[(2,8)] = dv_accelb[(2,2)];


      // this matrix is tranposed. reusing the quartenion change we calculated earlier cause it the same then x by the transposed 2.5 R equation, (swap rows and columns)
    fx[(3,3)] = quartenion_vec[0] * quartenion_vec[0] + quartenion_vec[1] * quartenion_vec[1] + quartenion_vec[2] * quartenion_vec[2] + quartenion_vec[3] * quartenion_vec[3];
    fx[(3,4)] = 2.0 * (quartenion_vec[1] * quartenion_vec[2] + quartenion_vec[0] * quartenion_vec[3]);
    fx[(3,5)] = 2.0 * (quartenion_vec[1] * quartenion_vec[3] - quartenion_vec[0] * quartenion_vec[3]);
    fx[(4,3)] = 2.0 * (quartenion_vec[1] * quartenion_vec[2] - quartenion_vec[0] * quartenion_vec[3]);
    fx[(4,4)] = quartenion_vec[0] * quartenion_vec[0] - quartenion_vec[1] * quartenion_vec[1] + quartenion_vec[2] * quartenion_vec[2] - quartenion_vec[3] * quartenion_vec[3];
    fx[(4,5)] = 2.0 * (quartenion_vec[2] * quartenion_vec[3] + quartenion_vec[0] * quartenion_vec[1]);
    fx[(5,3)] = 2.0 * (quartenion_vec[1] * quartenion_vec[3] + quartenion_vec[0] * quartenion_vec[2]);
    fx[(5,4)] = 2.0 * (quartenion_vec[2] * quartenion_vec[3] - quartenion_vec[0] * quartenion_vec[1]);
    fx[(5,5)] = quartenion_vec[0] * quartenion_vec[0] - quartenion_vec[1] * quartenion_vec[1] - quartenion_vec[2] * quartenion_vec[2] + quartenion_vec[3] * quartenion_vec[3];
      
    // gyro bias effect on rotation
    fx[(3,9)] = -1.0 * dt;
    fx[(4,10)] = -1.0 * dt;
    fx[(5,11)] = -1.0 * dt;
    
    fx[(6,6)] = 1.0;
    fx[(7,7)] = 1.0;
    fx[(8,8)] = 1.0;
    
    fx[(9,9)] = 1.0;
    fx[(10,10)] = 1.0;
    fx[(11,11)] = 1.0;

    let mut fi = ErrorCov::zeros();
     fi[(0,0)] = 1.0;   fi[(1,1)] = 1.0;   fi[(2,2)] = 1.0;
      fi[(3,3)] = 1.0;   fi[(4,4)] = 1.0;   fi[(5,5)] = 1.0;
       fi[(6,6)] = 1.0;   fi[(7,7)] = 1.0;   fi[(8,8)] = 1.0;
        fi[(9,9)] = 1.0;   fi[(10,10)] = 1.0;   fi[(11,11)] = 1.0;

    self.error_state_covariance = fx * self.error_state_covariance * fx.transpose() + fi * noise.process_noise * fi.transpose();



    }
}

impl Noise { // 12x12 matrix:  velocity, orientation or rotation, accel bias, gyro bias.
    fn noise_new() -> Self {
       Self { process_noise: {
            let mut noise = Process_Noise::zeros();
            noise[(0,0)] = 1.0;
            noise
        },
    }
    }
}




/*






*/