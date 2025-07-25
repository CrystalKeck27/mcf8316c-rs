//! Constants for register addresses

use arbitrary_int::u12;

/// ISD Configuration
pub const ISD_CONFIG: u12 = u12::new(0x0080);
/// Reverse Drive Configuration
pub const REV_DRIVE_CONFIG: u12 = u12::new(0x0082);
/// Motor Startup Configuration 1
pub const MOTOR_STARTUP1: u12 = u12::new(0x0084);
/// Motor Startup Configuration 2
pub const MOTOR_STARTUP2: u12 = u12::new(0x0086);
/// Closed Loop Configuration 1
pub const CLOSED_LOOP1: u12 = u12::new(0x0088);
/// Closed Loop Configuration 2
pub const CLOSED_LOOP2: u12 = u12::new(0x008A);
/// Closed Loop Configuration 3
pub const CLOSED_LOOP3: u12 = u12::new(0x008C);
/// Closed Loop Configuration 4
pub const CLOSED_LOOP4: u12 = u12::new(0x008E);
/// Reference Profile Configuration 1
pub const REF_PROFILES1: u12 = u12::new(0x0094);
/// Reference Profile Configuration 2
pub const REF_PROFILES2: u12 = u12::new(0x0096);
/// Reference Profile Configuration 3
pub const REF_PROFILES3: u12 = u12::new(0x0098);
/// Reference Profile Configuration 4
pub const REF_PROFILES4: u12 = u12::new(0x009A);
/// Reference Profile Configuration 5
pub const REF_PROFILES5: u12 = u12::new(0x009C);
/// Reference Profile Configuration 6
pub const REF_PROFILES6: u12 = u12::new(0x009E);
/// Fault Configuration 1
pub const FAULT_CONFIG1: u12 = u12::new(0x0090);
/// Fault Configuration 2
pub const FAULT_CONFIG2: u12 = u12::new(0x0092);
/// Internal Algorithm Configuration 1
pub const INT_ALGO_1: u12 = u12::new(0x00A0);
/// Internal Algorithm Configuration 2
pub const INT_ALGO_2: u12 = u12::new(0x00A2);
/// Hardware Pin Configuration
pub const PIN_CONFIG: u12 = u12::new(0x00A4);
/// Device Configuration 1
pub const DEVICE_CONFIG1: u12 = u12::new(0x00A6);
/// Device Configuration 2
pub const DEVICE_CONFIG2: u12 = u12::new(0x00A8);
/// Peripheral Configuration 1
pub const PERI_CONFIG1: u12 = u12::new(0x00AA);
/// Gate Driver Configuration 1
pub const GD_CONFIG1: u12 = u12::new(0x00AC);
/// Gate Driver Configuration 2
pub const GD_CONFIG2: u12 = u12::new(0x00AE);
/// Fault Status Register
pub const GATE_DRIVER_FAULT_STATUS: u12 = u12::new(0x00E0);
/// Fault Status Register
pub const CONTROLLER_FAULT_STATUS: u12 = u12::new(0x00E2);
/// System Status Register
pub const ALGO_STATUS: u12 = u12::new(0x00E4);
/// System Status Register
pub const MTR_PARAMS: u12 = u12::new(0x00E6);
/// System Status Register
pub const ALGO_STATUS_MPET: u12 = u12::new(0x00E8);
/// Device Control Register
pub const ALGO_CTRL1: u12 = u12::new(0x00EA);
/// Algorithm Control Register
pub const ALGO_DEBUG1: u12 = u12::new(0x00EC);
/// Algorithm Control Register
pub const ALGO_DEBUG2: u12 = u12::new(0x00EE);
/// Current PI Controller used
pub const CURRENT_PI: u12 = u12::new(0x00F0);
/// Speed PI controller used
pub const SPEED_PI: u12 = u12::new(0x00F2);
/// DAC1 Control Register
pub const DAC_1: u12 = u12::new(0x00F4);
/// DAC2 Control Register
pub const DAC_2: u12 = u12::new(0x00F6);
/// Current Algorithm State Register
pub const ALGORITHM_STATE: u12 = u12::new(0x0190);
/// FG Speed Feedback Register
pub const FG_SPEED_FDBK: u12 = u12::new(0x0196);
/// Calculated DC Bus Current Register
pub const BUS_CURRENT: u12 = u12::new(0x0410);
/// Measured Current on Phase A Register
pub const PHASE_CURRENT_A: u12 = u12::new(0x0440);
/// Measured Current on Phase B Register
pub const PHASE_CURRENT_B: u12 = u12::new(0x0442);
/// Measured Current on Phase C Register
pub const PHASE_CURRENT_C: u12 = u12::new(0x0444);
/// CSA Gain Register
pub const CSA_GAIN_FEEDBACK: u12 = u12::new(0x0468);
/// Voltage Gain Register
pub const VOLTAGE_GAIN_FEEDBACK: u12 = u12::new(0x0472);
/// VM Voltage Register
pub const VM_VOLTAGE: u12 = u12::new(0x0476);
/// Phase A Voltage Register
pub const PHASE_VOLTAGE_VA: u12 = u12::new(0x047A);
/// Phase B Voltage Register
pub const PHASE_VOLTAGE_VB: u12 = u12::new(0x047C);
/// Phase C Voltage Register
pub const PHASE_VOLTAGE_VC: u12 = u12::new(0x047E);
/// Sine of Commutation Angle
pub const SIN_COMMUTATION_ANGLE: u12 = u12::new(0x04B6);
/// Cosine of Commutation Angle
pub const COS_COMMUTATION_ANGLE: u12 = u12::new(0x04B8);
/// IALPHA Current Register
pub const IALPHA: u12 = u12::new(0x04D2);
/// IBETA Current Register
pub const IBETA: u12 = u12::new(0x04D4);
/// VALPHA Voltage Register
pub const VALPHA: u12 = u12::new(0x04D6);
/// VBETA Voltage Register
pub const VBETA: u12 = u12::new(0x04D8);
/// Measured d-axis Current Register
pub const ID: u12 = u12::new(0x04E2);
/// Measured q-axis Current Register
pub const IQ: u12 = u12::new(0x04E4);
/// VD Voltage Register
pub const VD: u12 = u12::new(0x04E6);
/// VQ Voltage Register
pub const VQ: u12 = u12::new(0x04E8);
/// Align Current Reference
pub const IQ_REF_ROTOR_ALIGN: u12 = u12::new(0x0524);
/// Open Loop Speed Register
pub const SPEED_REF_OPEN_LOOP: u12 = u12::new(0x053C);
/// Open Loop Current Reference
pub const IQ_REF_OPEN_LOOP: u12 = u12::new(0x054C);
/// Speed Reference Register
pub const SPEED_REF_CLOSED_LOOP: u12 = u12::new(0x05D4);
/// Reference for Current Loop Register
pub const ID_REF_CLOSED_LOOP: u12 = u12::new(0x0606);
/// Reference for Current Loop Register
pub const IQ_REF_CLOSED_LOOP: u12 = u12::new(0x0608);
/// ISD State Register
pub const ISD_STATE: u12 = u12::new(0x0682);
/// ISD Speed Register
pub const ISD_SPEED: u12 = u12::new(0x068C);
/// IPD State Register
pub const IPD_STATE: u12 = u12::new(0x06C0);
/// Calculated IPD Angle Register
pub const IPD_ANGLE: u12 = u12::new(0x0704);
/// Estimated BEMF EQ Register
pub const ED: u12 = u12::new(0x074A);
/// Estimated BEMF ED Register
pub const EQ: u12 = u12::new(0x074C);
/// Speed Feedback Register
pub const SPEED_FDBK: u12 = u12::new(0x075A);
/// Estimated rotor Position Register
pub const THETA_EST: u12 = u12::new(0x075E);
