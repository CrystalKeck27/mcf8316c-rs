pub const ISD_CONFIG: u16 = 0x0080;
pub const REV_DRIVE_CONFIG: u16 = 0x0082;
pub const MOTOR_STARTUP1: u16 = 0x0084;
pub const MOTOR_STARTUP2: u16 = 0x0086;
pub const CLOSED_LOOP1: u16 = 0x0088;
pub const CLOSED_LOOP2: u16 = 0x008A;
pub const CLOSED_LOOP3: u16 = 0x008C;
pub const CLOSED_LOOP4: u16 = 0x008E;
pub const REF_PROFILES1: u16 = 0x0094;
pub const REF_PROFILES2: u16 = 0x0096;
pub const REF_PROFILES3: u16 = 0x0098;
pub const REF_PROFILES4: u16 = 0x009A;
pub const REF_PROFILES5: u16 = 0x009C;
pub const REF_PROFILES6: u16 = 0x009E;
pub const FAULT_CONFIG1: u16 = 0x0090;
pub const FAULT_CONFIG2: u16 = 0x0092;
pub const INT_ALGO_1: u16 = 0x00A0;
pub const INT_ALGO_2: u16 = 0x00A2;
pub const PIN_CONFIG: u16 = 0x00A4;
pub const DEVICE_CONFIG1: u16 = 0x00A6;
pub const DEVICE_CONFIG2: u16 = 0x00A8;
pub const PERI_CONFIG1: u16 = 0x00AA;
pub const GD_CONFIG1: u16 = 0x00AC;
pub const GD_CONFIG2: u16 = 0x00AE;
pub const GATE_DRIVER_FAULT_STATUS: u16 = 0x00E0;
pub const CONTROLLER_FAULT_STATUS: u16 = 0x00E2;
pub const ALGO_STATUS: u16 = 0x00E4;
pub const MTR_PARAMS: u16 = 0x00E6;
pub const ALGO_STATUS_MPET: u16 = 0x00E8;
pub const ALGO_CTRL1: u16 = 0x00EA;
pub const ALGO_DEBUG1: u16 = 0x00EC;
pub const ALGO_DEBUG2: u16 = 0x00EE;
pub const CURRENT_PI: u16 = 0x00F0;
pub const SPEED_PI: u16 = 0x00F2;
pub const DAC_1: u16 = 0x00F4;
pub const DAC_2: u16 = 0x00F6;
pub const ALGORITHM_STATE: u16 = 0x0190;
pub const FG_SPEED_FDBK: u16 = 0x0196;
pub const BUS_CURRENT: u16 = 0x0410;
pub const PHASE_CURRENT_A: u16 = 0x0440;
pub const PHASE_CURRENT_B: u16 = 0x0442;
pub const PHASE_CURRENT_C: u16 = 0x0444;
pub const CSA_GAIN_FEEDBACK: u16 = 0x0468;
pub const VOLTAGE_GAIN_FEEDBACK: u16 = 0x0472;
pub const VM_VOLTAGE: u16 = 0x0476;
pub const PHASE_VOLTAGE_VA: u16 = 0x047A;
pub const PHASE_VOLTAGE_VB: u16 = 0x047C;
pub const PHASE_VOLTAGE_VC: u16 = 0x047E;
pub const SIN_COMMUTATION_ANGLE: u16 = 0x04B6;
pub const COS_COMMUTATION_ANGLE: u16 = 0x04B8;
pub const IALPHA: u16 = 0x04D2;
pub const IBETA: u16 = 0x04D4;
pub const VALPHA: u16 = 0x04D6;
pub const VBETA: u16 = 0x04D8;
pub const ID: u16 = 0x04E2;
pub const IQ: u16 = 0x04E4;
pub const VD: u16 = 0x04E6;
pub const VQ: u16 = 0x04E8;
pub const IQ_REF_ROTOR_ALIGN: u16 = 0x0524;
pub const SPEED_REF_OPEN_LOOP: u16 = 0x053C;
pub const IQ_REF_OPEN_LOOP: u16 = 0x054C;
pub const SPEED_REF_CLOSED_LOOP: u16 = 0x05D4;
pub const ID_REF_CLOSED_LOOP: u16 = 0x0606;
pub const IQ_REF_CLOSED_LOOP: u16 = 0x0608;
pub const ISD_STATE: u16 = 0x0682;
pub const ISD_SPEED: u16 = 0x068C;
pub const IPD_STATE: u16 = 0x06C0;
pub const IPD_ANGLE: u16 = 0x0704;
pub const ED: u16 = 0x074A;
pub const EQ: u16 = 0x074C;
pub const SPEED_FDBK: u16 = 0x075A;
pub const THETA_EST: u16 = 0x075E;
