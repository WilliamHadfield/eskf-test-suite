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

    }
    fn predict (&mut self, imu: ImuMeasVec) {
    let mut true_ax = imu[0] - self.nominal_state[7];
     let mut true_ay = imu[1] - self.nominal_state[8];
      let mut true_az = imu[2] - self.nominal_state[9];

      
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