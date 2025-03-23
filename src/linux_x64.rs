use std::os::raw::c_char;
use std::os::raw::c_uchar;
use std::os::raw::c_uint;
use std::os::raw::c_ushort;
use std::os::raw::c_void;


pub type DWORD = c_uint;
pub type ULONG = c_uint;
pub type USHORT = c_ushort;
pub type UCHAR = c_uchar;
pub type WORD = c_ushort;
pub type BYTE = c_uchar;
pub type BOOL = c_uint;
pub type CHAR = c_uchar;
pub type PUCHAR = *mut UCHAR;
pub type PCHAR = *mut c_char;
pub type PVOID = *mut c_void;
pub type HANDLE = *mut c_void;
pub type LONG = c_uint;
pub type UINT = c_uint;
pub type LPCTSTR = *const c_char;
pub type LPDWORD = *mut DWORD;
pub type LPWORD = *mut WORD;
pub type PULONG = *mut ULONG;
pub type LPLONG = *mut LONG;
pub type LPVOID = PVOID;
pub type FT_HANDLE = PVOID;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT_STATUS {
    FT_OK = 0,
    FT_INVALID_HANDLE = 1,
    FT_DEVICE_NOT_FOUND = 2,
    FT_DEVICE_NOT_OPENED = 3,
    FT_IO_ERROR = 4,
    FT_INSUFFICIENT_RESOURCES = 5,
    FT_INVALID_PARAMETER = 6,
    FT_INVALID_BAUD_RATE = 7,
    FT_DEVICE_NOT_OPENED_FOR_ERASE = 8,
    FT_DEVICE_NOT_OPENED_FOR_WRITE = 9,
    FT_FAILED_TO_WRITE_DEVICE = 10,
    FT_EEPROM_READ_FAILED = 11,
    FT_EEPROM_WRITE_FAILED = 12,
    FT_EEPROM_ERASE_FAILED = 13,
    FT_EEPROM_NOT_PRESENT = 14,
    FT_EEPROM_NOT_PROGRAMMED = 15,
    FT_INVALID_ARGS = 16,
    FT_NOT_SUPPORTED = 17,
    FT_OTHER_ERROR = 18,
    FT_DEVICE_LIST_NOT_READY = 19,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT_DEVICE {
    FT_DEVICE_BM = 0,
    FT_DEVICE_AM = 1,
    FT_DEVICE_100AX = 2,
    FT_DEVICE_UNKNOWN = 3,
    FT_DEVICE_2232C = 4,
    FT_DEVICE_232R = 5,
    FT_DEVICE_2232H = 6,
    FT_DEVICE_4232H = 7,
    FT_DEVICE_232H = 8,
    FT_DEVICE_X_SERIES = 9,
    FT_DEVICE_4222H_0 = 10,
    FT_DEVICE_4222H_1_2 = 11,
    FT_DEVICE_4222H_3 = 12,
    FT_DEVICE_4222_PROG = 13,
    FT_DEVICE_900 = 14,
    FT_DEVICE_930 = 15,
    FT_DEVICE_UMFTPD3A = 16,
    FT_DEVICE_2233HP = 17,
    FT_DEVICE_4233HP = 18,
    FT_DEVICE_2232HP = 19,
    FT_DEVICE_4232HP = 20,
    FT_DEVICE_233HP = 21,
    FT_DEVICE_232HP = 22,
    FT_DEVICE_2232HA = 23,
    FT_DEVICE_4232HA = 24,
}

impl From<u32> for FT_DEVICE {
    fn from(value: u32) -> Self {
        match value {
            0 => FT_DEVICE::FT_DEVICE_BM,
            1 => FT_DEVICE::FT_DEVICE_AM,
            2 => FT_DEVICE::FT_DEVICE_100AX,
            3 => FT_DEVICE::FT_DEVICE_UNKNOWN,
            4 => FT_DEVICE::FT_DEVICE_2232C,
            5 => FT_DEVICE::FT_DEVICE_232R,
            6 => FT_DEVICE::FT_DEVICE_2232H,
            7 => FT_DEVICE::FT_DEVICE_4232H,
            8 => FT_DEVICE::FT_DEVICE_232H,
            9 => FT_DEVICE::FT_DEVICE_X_SERIES,
            10 => FT_DEVICE::FT_DEVICE_4222H_0,
            11 => FT_DEVICE::FT_DEVICE_4222H_1_2,
            12 => FT_DEVICE::FT_DEVICE_4222H_3,
            13 => FT_DEVICE::FT_DEVICE_4222_PROG,
            14 => FT_DEVICE::FT_DEVICE_900,
            15 => FT_DEVICE::FT_DEVICE_930,
            16 => FT_DEVICE::FT_DEVICE_UMFTPD3A,
            17 => FT_DEVICE::FT_DEVICE_2233HP,
            18 => FT_DEVICE::FT_DEVICE_4233HP,
            19 => FT_DEVICE::FT_DEVICE_2232HP,
            20 => FT_DEVICE::FT_DEVICE_4232HP,
            21 => FT_DEVICE::FT_DEVICE_233HP,
            22 => FT_DEVICE::FT_DEVICE_232HP,
            23 => FT_DEVICE::FT_DEVICE_2232HA,
            24 => FT_DEVICE::FT_DEVICE_4232HA,
            _ => panic!("Invalid value for FT_DEVICE: {}", value),
        }
    }
}



//FT_OpenEx Flags
pub enum Ft_OpenFlag {
    FT_OPEN_BY_SERIAL_NUMBER = 1,
    FT_OPEN_BY_DESCRIPTION = 2,
    FT_OPEN_BY_LOCATION = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct _ft_device_list_info_node {
    pub Flags: ULONG,
    pub Type: ULONG,
    pub ID: ULONG,
    pub LocId: DWORD,
    pub SerialNumber: [c_char; 16usize],
    pub Description: [c_char; 64usize],
    pub ftHandle: FT_HANDLE,
}
pub type FT_DEVICE_LIST_INFO_NODE = _ft_device_list_info_node;


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT4222_STATUS {
    FT4222_OK = 0,
    FT4222_INVALID_HANDLE = 1,
    FT4222_DEVICE_NOT_FOUND = 2,
    FT4222_DEVICE_NOT_OPENED = 3,
    FT4222_IO_ERROR = 4,
    FT4222_INSUFFICIENT_RESOURCES = 5,
    FT4222_INVALID_PARAMETER = 6,
    FT4222_INVALID_BAUD_RATE = 7,
    FT4222_DEVICE_NOT_OPENED_FOR_ERASE = 8,
    FT4222_DEVICE_NOT_OPENED_FOR_WRITE = 9,
    FT4222_FAILED_TO_WRITE_DEVICE = 10,
    FT4222_EEPROM_READ_FAILED = 11,
    FT4222_EEPROM_WRITE_FAILED = 12,
    FT4222_EEPROM_ERASE_FAILED = 13,
    FT4222_EEPROM_NOT_PRESENT = 14,
    FT4222_EEPROM_NOT_PROGRAMMED = 15,
    FT4222_INVALID_ARGS = 16,
    FT4222_NOT_SUPPORTED = 17,
    FT4222_OTHER_ERROR = 18,
    FT4222_DEVICE_LIST_NOT_READY = 19,

    FT4222_DEVICE_NOT_SUPPORTED = 1000, // FT_STATUS extending message
    FT4222_CLK_NOT_SUPPORTED = 1001,
    FT4222_VENDER_CMD_NOT_SUPPORTED = 1002,
    FT4222_IS_NOT_SPI_MODE = 1003,
    FT4222_IS_NOT_I2C_MODE = 1004,
    FT4222_IS_NOT_SPI_SINGLE_MODE = 1005,
    FT4222_IS_NOT_SPI_MULTI_MODE = 1006,
    FT4222_WRONG_I2C_ADDR = 1007,
    FT4222_INVAILD_FUNCTION = 1008,
    FT4222_INVALID_POINTER = 1009,
    FT4222_EXCEEDED_MAX_TRANSFER_SIZE = 1010,
    FT4222_FAILED_TO_READ_DEVICE = 1011,
    FT4222_I2C_NOT_SUPPORTED_IN_THIS_MODE = 1012,
    FT4222_GPIO_NOT_SUPPORTED_IN_THIS_MODE = 1013,
    FT4222_GPIO_EXCEEDED_MAX_PORTNUM = 1014,
    FT4222_GPIO_WRITE_NOT_SUPPORTED = 1015,
    FT4222_GPIO_PULLUP_INVALID_IN_INPUTMODE = 1016,
    FT4222_GPIO_PULLDOWN_INVALID_IN_INPUTMODE = 1017,
    FT4222_GPIO_OPENDRAIN_INVALID_IN_OUTPUTMODE = 1018,
    FT4222_INTERRUPT_NOT_SUPPORTED = 1019,
    FT4222_GPIO_INPUT_NOT_SUPPORTED = 1020,
    FT4222_EVENT_NOT_SUPPORTED = 1021,
    FT4222_FUN_NOT_SUPPORT = 1022,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT4222_ClockRate {
    SYS_CLK_60 = 0,      // 60 MHz
    SYS_CLK_24 = 1,      // 24 MHz
    SYS_CLK_48 = 2,      // 48 MHz
    SYS_CLK_80 = 3,      // 80 MHz
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT4222_FUNCTION {
    FT4222_I2C_MASTER = 1,
    FT4222_I2C_SLAVE = 2,
    FT4222_SPI_MASTER = 3,
    FT4222_SPI_SLAVE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT4222_SPIMode {
    SPI_IO_NONE = 0,
    SPI_IO_SINGLE = 1,
    SPI_IO_DUAL = 2,
    SPI_IO_QUAD = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT4222_SPIClock {
    CLK_NONE = 0,
    CLK_DIV_2 = 1,      // 1/2   System Clock
    CLK_DIV_4 = 2,      // 1/4   System Clock
    CLK_DIV_8 = 3,      // 1/8   System Clock
    CLK_DIV_16 = 4,     // 1/16  System Clock
    CLK_DIV_32 = 5,     // 1/32  System Clock
    CLK_DIV_64 = 6,     // 1/64  System Clock
    CLK_DIV_128 = 7,    // 1/128 System Clock
    CLK_DIV_256 = 8,    // 1/256 System Clock
    CLK_DIV_512 = 9,    // 1/512 System Clock
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT4222_SPICPOL {
    CLK_IDLE_LOW = 0,
    CLK_IDLE_HIGH = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FT4222_SPICPHA {
    CLK_LEADING = 0,
    CLK_TRAILING = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SPI_DrivingStrength {
    DS_4MA = 0,
    DS_8MA = 1,
    DS_12MA = 2,
    DS_16MA = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SPI_ChipSelect {
    CS_ACTIVE_LOW = 0,
    CS_ACTIVE_HIGH = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GPIO_Port {
    GPIO_PORT0 = 0,
    GPIO_PORT1 = 1,
    GPIO_PORT2 = 2,
    GPIO_PORT3 = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GPIO_Dir {
    GPIO_OUTPUT = 0,
    GPIO_INPUT = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GPIO_Trigger {
    GPIO_TRIGGER_RISING = 0x01,
    GPIO_TRIGGER_FALLING = 0x02,
    GPIO_TRIGGER_LEVEL_HIGH = 0x04,
    GPIO_TRIGGER_LEVEL_LOW = 0x08,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GPIO_Output {
    GPIO_OUTPUT_LOW = 0,
    GPIO_OUTPUT_HIGH = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum I2C_MasterFlag {
    NONE = 0x80,
    START = 0x02,
    Repeated_START = 0x03, // Repeated_START will not send master code in HS mode
    STOP = 0x04,
    START_AND_STOP = 0x06, // START condition followed by SEND and STOP condition
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SPI_SlaveProtocol {
    SPI_SLAVE_WITH_PROTOCOL = 0,
    SPI_SLAVE_NO_PROTOCOL = 1,
    SPI_SLAVE_NO_ACK = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FT4222_Version {
    pub chipVersion: DWORD,
    pub dllVersion: DWORD,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SPI_Slave_Header {
    pub syncWord: u8,
    pub cmd: u8,
    pub sn: u8,
    pub size: u16,
}

unsafe extern "C" {
    fn FT_CreateDeviceInfoList(lpdwNumDevs: LPDWORD) -> FT_STATUS;
    fn FT_GetDeviceInfoDetail(
        dwIndex: DWORD,
        lpdwFlags: LPDWORD,
        lpdwType: LPDWORD,
        lpdwID: LPDWORD,
        lpdwLocId: LPDWORD,
        lpSerialNumber: LPVOID,
        lpDescription: LPVOID,
        pftHandle: *mut FT_HANDLE,
    ) -> FT_STATUS;
    fn FT_OpenEx(pvArg1: PVOID, dwFlags: DWORD, pHandle: *mut FT_HANDLE) -> FT_STATUS;
    fn FT_Close(ftHandle: FT_HANDLE) -> FT_STATUS;

    fn FT4222_UnInitialize(ftHandle: FT_HANDLE) -> FT4222_STATUS;
    fn FT4222_SetClock(ftHandle: FT_HANDLE , clk: FT4222_ClockRate) -> FT4222_STATUS;
    fn FT4222_GetClock(ftHandle: FT_HANDLE, clk: *mut FT4222_ClockRate) -> FT4222_STATUS;
    fn FT4222_SetWakeUpInterrupt(ftHandle: FT_HANDLE, enable: BOOL) -> FT4222_STATUS;
    fn FT4222_SetInterruptTrigger(ftHandle: FT_HANDLE, trigger: GPIO_Trigger) -> FT4222_STATUS;
    fn FT4222_SetSuspendOut(ftHandle: FT_HANDLE, enable: BOOL) -> FT4222_STATUS;
    fn FT4222_GetMaxTransferSize(ftHandle: FT_HANDLE, pMaxSize: *mut u16) -> FT4222_STATUS;
    fn FT4222_SetEventNotification(ftHandle: FT_HANDLE, mask: DWORD, param: PVOID) -> FT4222_STATUS;
    fn FT4222_GetVersion(ftHandle: FT_HANDLE, pVersion: *mut FT4222_Version) -> FT4222_STATUS;
    fn FT4222_GetChipMode(ftHandle: FT_HANDLE, pChipMode: *mut u8) -> FT4222_STATUS;
    fn FT4222_ChipReset(ftHandle: FT_HANDLE) -> FT4222_STATUS;

    // FT4222 SPI Functions
    fn FT4222_SPIMaster_Init(ftHandle: FT_HANDLE, ioLine: FT4222_SPIMode, clock: FT4222_SPIClock, cpol: FT4222_SPICPOL, cpha: FT4222_SPICPHA, ssoMap: u8) -> FT4222_STATUS;
    fn FT4222_SPIMaster_SetMode(ftHandle: FT_HANDLE, cpol: FT4222_SPICPOL, cpha: FT4222_SPICPHA) -> FT4222_STATUS;
    fn FT4222_SPIMaster_SetCS(ftHandle: FT_HANDLE, cs: SPI_ChipSelect) -> FT4222_STATUS;
    fn FT4222_SPIMaster_SetLines(ftHandle: FT_HANDLE, spiMode: FT4222_SPIMode) -> FT4222_STATUS;
    fn FT4222_SPIMaster_SingleRead(ftHandle: FT_HANDLE, buffer: *mut u8, bufferSize: u16, sizeOfRead: *mut u16, isEndTransaction: BOOL) -> FT4222_STATUS;
    fn FT4222_SPIMaster_SingleWrite(ftHandle: FT_HANDLE, buffer: *const u8, bufferSize: u16, sizeTransferred: *mut u16, isEndTransaction: BOOL) -> FT4222_STATUS;
    fn FT4222_SPIMaster_SingleReadWrite(ftHandle: FT_HANDLE, readBuffer: *mut u8, writeBuffer: *const u8, bufferSize: u16, sizeTransferred: *mut u16, isEndTransaction: BOOL) -> FT4222_STATUS;
    fn FT4222_SPIMaster_MultiReadWrite(ftHandle: FT_HANDLE, readBuffer: *mut u8, writeBuffer: *const u8, singleWriteBytes: u8, multiWriteBytes: u16, multiReadBytes: u16, sizeOfRead: *mut u32) -> FT4222_STATUS;

    fn FT4222_SPISlave_Init(ftHandle: FT_HANDLE) -> FT4222_STATUS;
    fn FT4222_SPISlave_InitEx(ftHandle: FT_HANDLE, protocolOpt: SPI_SlaveProtocol) -> FT4222_STATUS;
    fn FT4222_SPISlave_SetMode(ftHandle: FT_HANDLE, cpol: FT4222_SPICPOL, cpha: FT4222_SPICPHA) -> FT4222_STATUS;
    fn FT4222_SPISlave_GetRxStatus(ftHandle: FT_HANDLE, pRxSize: *mut u16) -> FT4222_STATUS;
    fn FT4222_SPISlave_Read(ftHandle: FT_HANDLE, buffer: *mut u8, bufferSize: u16, sizeOfRead: *mut u16) -> FT4222_STATUS;
    fn FT4222_SPISlave_Write(ftHandle: FT_HANDLE, buffer: *const u8, bufferSize: u16, sizeTransferred: *mut u16) -> FT4222_STATUS;

    fn FT4222_SPI_Reset(ftHandle: FT_HANDLE) -> FT4222_STATUS;
    fn FT4222_SPI_ResetTransaction(ftHandle: FT_HANDLE, spiIdx: u8) -> FT4222_STATUS;
    fn FT4222_SPI_SetDrivingStrength(ftHandle: FT_HANDLE, clkStrength: SPI_DrivingStrength, ioStrength: SPI_DrivingStrength, ssoStrength: SPI_DrivingStrength) -> FT4222_STATUS;

    // FT4222 I2C Functions
    fn FT4222_I2CMaster_Init(ftHandle: FT_HANDLE, kbps: u32) -> FT4222_STATUS;
    fn FT4222_I2CMaster_Read(ftHandle: FT_HANDLE, deviceAddress: u16, buffer: *mut u8, bufferSize: u16, sizeTransferred: *mut u16) -> FT4222_STATUS;
    fn FT4222_I2CMaster_Write(ftHandle: FT_HANDLE, deviceAddress: u16, buffer: *const u8, bufferSize: u16, sizeTransferred: *mut u16) -> FT4222_STATUS;
    fn FT4222_I2CMaster_ReadEx(ftHandle: FT_HANDLE, deviceAddress: u16, flag: u8, buffer: *mut u8, bufferSize: u16, sizeTransferred: *mut u16) -> FT4222_STATUS;
    fn FT4222_I2CMaster_WriteEx(ftHandle: FT_HANDLE, deviceAddress: u16, flag: u8, buffer: *const u8, bufferSize: u16, sizeTransferred: *mut u16) -> FT4222_STATUS;
    fn FT4222_I2CMaster_Reset(ftHandle: FT_HANDLE) -> FT4222_STATUS;
    fn FT4222_I2CMaster_GetStatus(ftHandle: FT_HANDLE, controllerStatus: *mut u8) -> FT4222_STATUS;
    fn FT4222_I2CMaster_ResetBus(ftHandle: FT_HANDLE) -> FT4222_STATUS;

    fn FT4222_I2CSlave_Init(ftHandle: FT_HANDLE) -> FT4222_STATUS;
    fn FT4222_I2CSlave_Reset(ftHandle: FT_HANDLE) -> FT4222_STATUS;
    fn FT4222_I2CSlave_GetAddress(ftHandle: FT_HANDLE, addr: *mut u8) -> FT4222_STATUS;
    fn FT4222_I2CSlave_SetAddress(ftHandle: FT_HANDLE, addr: u8) -> FT4222_STATUS;
    fn FT4222_I2CSlave_GetRxStatus(ftHandle: FT_HANDLE, pRxSize: *mut u16) -> FT4222_STATUS;
    fn FT4222_I2CSlave_Read(ftHandle: FT_HANDLE, buffer: *mut u8, bufferSize: u16, sizeTransferred: *mut u16) -> FT4222_STATUS;
    fn FT4222_I2CSlave_Write(ftHandle: FT_HANDLE, buffer:*const u8, bufferSize: u16, sizeTransferred: *mut u16) -> FT4222_STATUS;
    fn FT4222_I2CSlave_SetClockStretch(ftHandle: FT_HANDLE, enable: BOOL) -> FT4222_STATUS;
    fn FT4222_I2CSlave_SetRespWord(ftHandle: FT_HANDLE, responseWord: u8) -> FT4222_STATUS;

    // FT4222 GPIO Functions
    fn FT4222_GPIO_Init(ftHandle: FT_HANDLE, gpioDir: *const GPIO_Dir) -> FT4222_STATUS;
    fn FT4222_GPIO_Read(ftHandle: FT_HANDLE, portNum: GPIO_Port , value: *mut BOOL) -> FT4222_STATUS;
    fn FT4222_GPIO_Write(ftHandle: FT_HANDLE, portNum: GPIO_Port , bValue: BOOL) -> FT4222_STATUS;
    fn FT4222_GPIO_SetInputTrigger(ftHandle: FT_HANDLE, portNum: GPIO_Port , trigger: GPIO_Trigger) -> FT4222_STATUS;
    fn FT4222_GPIO_GetTriggerStatus(ftHandle: FT_HANDLE, portNum: GPIO_Port , queueSize: *mut u16) -> FT4222_STATUS;
    fn FT4222_GPIO_ReadTriggerQueue(ftHandle: FT_HANDLE, portNum: GPIO_Port , events: *mut GPIO_Trigger, readSize: u16, sizeofRead: *mut u16) -> FT4222_STATUS;
    fn FT4222_GPIO_SetWaveFormMode(ftHandle: FT_HANDLE, enable: BOOL) -> FT4222_STATUS;
}

//=================== 安全封装 =================
pub fn create_device_info_list() -> Result<u32, FT_STATUS> {
    let mut num_devs: u32 = 0;
    let status = unsafe { FT_CreateDeviceInfoList(&mut num_devs as LPDWORD) };
    if status == FT_STATUS::FT_OK {
        Ok(num_devs)
    } else {
        Err(status)
    }
}

pub fn get_device_info_detail(index: u32) -> Result<(u32, FT_DEVICE, u32, u32, String, String, FT_HANDLE), FT_STATUS> {
    let mut flags: u32 = 0;
    let mut type_: u32 = 0;
    let mut id: u32 = 0;
    let mut loc_id: u32 = 0;
    let mut serial_number = [0 as c_char; 16];
    let mut description = [0 as c_char; 64];
    let mut handle: FT_HANDLE = std::ptr::null_mut();

    let status = unsafe {
        FT_GetDeviceInfoDetail(
            index,
            &mut flags,
            &mut type_,
            &mut id,
            &mut loc_id,
            serial_number.as_mut_ptr() as LPVOID,
            description.as_mut_ptr() as LPVOID,
            &mut handle,
        )
    };

    if status == FT_STATUS::FT_OK {
        let serial = unsafe { std::ffi::CStr::from_ptr(serial_number.as_ptr()).to_string_lossy().into_owned() };
        let desc = unsafe { std::ffi::CStr::from_ptr(description.as_ptr()).to_string_lossy().into_owned() };
        Ok((flags, type_.into(), id, loc_id, serial, desc, handle))
    } else {
        Err(status)
    }
}

pub fn open_ex_by_location(local_id: u32) -> Result<FT_HANDLE, FT_STATUS> {
    let mut handle: FT_HANDLE = std::ptr::null_mut();
    let status = unsafe {
        FT_OpenEx(
            local_id as *mut c_void, // 将位置 ID 转换为 PVOID
            Ft_OpenFlag::FT_OPEN_BY_LOCATION as u32, // 固定使用 FT_OPEN_BY_LOCATION
            &mut handle,
        )
    };

    if status == FT_STATUS::FT_OK {
        Ok(handle)
    } else {
        Err(status)
    }
}


pub fn close(handle: FT_HANDLE) -> Result<(), FT_STATUS> {
    let status = unsafe { FT_Close(handle) };
    if status == FT_STATUS::FT_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn uninitialize(handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_UnInitialize(handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn set_clock(handle: FT_HANDLE, clock: FT4222_ClockRate) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SetClock(handle, clock) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn get_clock(handle: FT_HANDLE) -> Result<FT4222_ClockRate, FT4222_STATUS> {
    let mut clock = FT4222_ClockRate::SYS_CLK_60;
    let status = unsafe { FT4222_GetClock(handle, &mut clock) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(clock)
    } else {
        Err(status)
    }
}

pub fn set_wake_up_interrupt(ft_handle: FT_HANDLE, enable: bool) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SetWakeUpInterrupt(ft_handle, enable as BOOL) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn set_interrupt_trigger(ft_handle: FT_HANDLE, trigger: GPIO_Trigger) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SetInterruptTrigger(ft_handle, trigger) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn set_suspend_out(ft_handle: FT_HANDLE, enable: bool) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SetSuspendOut(ft_handle, enable as BOOL) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn get_max_transfer_size(ft_handle: FT_HANDLE) -> Result<u16, FT4222_STATUS> {
    let mut max_size: u16 = 0;
    let status = unsafe { FT4222_GetMaxTransferSize(ft_handle, &mut max_size) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(max_size)
    } else {
        Err(status)
    }
}

pub fn set_event_notification(ft_handle: FT_HANDLE, mask: u32, param: *mut c_void) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SetEventNotification(ft_handle, mask, param) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn get_version(ft_handle: FT_HANDLE) -> Result<FT4222_Version, FT4222_STATUS> {
    let mut version = FT4222_Version {
        chipVersion: 0,
        dllVersion: 0,
    };
    let status = unsafe { FT4222_GetVersion(ft_handle, &mut version) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(version)
    } else {
        Err(status)
    }
}

pub fn get_chip_mode(ft_handle: FT_HANDLE) -> Result<u8, FT4222_STATUS> {
    let mut chip_mode: u8 = 0;
    let status = unsafe { FT4222_GetChipMode(ft_handle, &mut chip_mode) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(chip_mode)
    } else {
        Err(status)
    }
}

pub fn chip_reset(ft_handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_ChipReset(ft_handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_master_init(
    handle: FT_HANDLE,
    io_line: FT4222_SPIMode,
    clock: FT4222_SPIClock,
    cpol: FT4222_SPICPOL,
    cpha: FT4222_SPICPHA,
    sso_map: u8
) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPIMaster_Init(handle, io_line, clock, cpol, cpha, sso_map) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_master_set_mode(ft_handle: FT_HANDLE, cpol: FT4222_SPICPOL, cpha: FT4222_SPICPHA) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPIMaster_SetMode(ft_handle, cpol, cpha) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_master_set_cs(ft_handle: FT_HANDLE, cs: SPI_ChipSelect) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPIMaster_SetCS(ft_handle, cs) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_master_set_lines(ft_handle: FT_HANDLE, spi_mode: FT4222_SPIMode) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPIMaster_SetLines(ft_handle, spi_mode) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_master_single_read(handle: FT_HANDLE, buffer: &mut [u8], is_end_transaction: bool) -> Result<usize, FT4222_STATUS> {
    let mut size_read: u16 = 0;
    let status = unsafe {
        FT4222_SPIMaster_SingleRead(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as u16,
            &mut size_read,
            is_end_transaction as BOOL,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_read as usize)
    } else {
        Err(status)
    }
}

pub fn spi_master_single_write(ft_handle: FT_HANDLE, buffer: &[u8], is_end_transaction: bool) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_SPIMaster_SingleWrite(
            ft_handle,
            buffer.as_ptr(),
            buffer.len() as u16,
            &mut size_transferred,
            is_end_transaction as BOOL,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn spi_master_single_read_write(ft_handle: FT_HANDLE, write_buffer: &[u8], read_buffer: &mut [u8], is_end_transaction: bool) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_SPIMaster_SingleReadWrite(
            ft_handle,
            read_buffer.as_mut_ptr(),
            write_buffer.as_ptr(),
            write_buffer.len() as u16,
            &mut size_transferred,
            is_end_transaction as BOOL,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn spi_master_multi_read_write(ft_handle: FT_HANDLE, write_buffer: &[u8], read_buffer: &mut [u8], single_write_bytes: u8, multi_write_bytes: u16, multi_read_bytes: u16) -> Result<usize, FT4222_STATUS> {
    let mut size_of_read: u32 = 0;
    let status = unsafe {
        FT4222_SPIMaster_MultiReadWrite(
            ft_handle,
            read_buffer.as_mut_ptr(),
            write_buffer.as_ptr(),
            single_write_bytes,
            multi_write_bytes,
            multi_read_bytes,
            &mut size_of_read,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_of_read as usize)
    } else {
        Err(status)
    }
}

pub fn spi_slave_init(ft_handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPISlave_Init(ft_handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_slave_init_ex(ft_handle: FT_HANDLE, protocol_opt: SPI_SlaveProtocol) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPISlave_InitEx(ft_handle, protocol_opt) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_slave_set_mode(ft_handle: FT_HANDLE, cpol: FT4222_SPICPOL, cpha: FT4222_SPICPHA) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPISlave_SetMode(ft_handle, cpol, cpha) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_slave_get_rx_status(ft_handle: FT_HANDLE) -> Result<u16, FT4222_STATUS> {
    let mut rx_size: u16 = 0;
    let status = unsafe { FT4222_SPISlave_GetRxStatus(ft_handle, &mut rx_size) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(rx_size)
    } else {
        Err(status)
    }
}

pub fn spi_slave_read(ft_handle: FT_HANDLE, buffer: &mut [u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_of_read: u16 = 0;
    let status = unsafe { FT4222_SPISlave_Read(ft_handle, buffer.as_mut_ptr(), buffer.len() as u16, &mut size_of_read) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_of_read as usize)
    } else {
        Err(status)
    }
}

pub fn spi_slave_write(ft_handle: FT_HANDLE, buffer: &[u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe { FT4222_SPISlave_Write(ft_handle, buffer.as_ptr(), buffer.len() as u16, &mut size_transferred) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn spi_reset(ft_handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPI_Reset(ft_handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_reset_transaction(ft_handle: FT_HANDLE, spi_idx: u8) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPI_ResetTransaction(ft_handle, spi_idx) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn spi_set_driving_strength(ft_handle: FT_HANDLE, clk_strength: SPI_DrivingStrength, io_strength: SPI_DrivingStrength, sso_strength: SPI_DrivingStrength) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_SPI_SetDrivingStrength(ft_handle, clk_strength, io_strength, sso_strength) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_master_init(handle: FT_HANDLE, kbps: u32) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CMaster_Init(handle, kbps) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_master_read(handle: FT_HANDLE, device_address: u16, buffer: &mut [u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_I2CMaster_Read(
            handle,
            device_address,
            buffer.as_mut_ptr(),
            buffer.len() as u16,
            &mut size_transferred,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn i2c_master_write(ft_handle: FT_HANDLE, device_address: u16, buffer: &[u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_I2CMaster_Write(
            ft_handle,
            device_address,
            buffer.as_ptr(),
            buffer.len() as u16,
            &mut size_transferred,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn i2c_master_read_ex(ft_handle: FT_HANDLE, device_address: u16, flag: u8, buffer: &mut [u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_I2CMaster_ReadEx(
            ft_handle,
            device_address,
            flag,
            buffer.as_mut_ptr(),
            buffer.len() as u16,
            &mut size_transferred,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn i2c_master_write_ex(ft_handle: FT_HANDLE, device_address: u16, flag: u8, buffer: &[u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_I2CMaster_WriteEx(
            ft_handle,
            device_address,
            flag,
            buffer.as_ptr(),
            buffer.len() as u16,
            &mut size_transferred,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn i2c_master_reset(ft_handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CMaster_Reset(ft_handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_master_get_status(ft_handle: FT_HANDLE) -> Result<u8, FT4222_STATUS> {
    let mut controller_status: u8 = 0;
    let status = unsafe { FT4222_I2CMaster_GetStatus(ft_handle, &mut controller_status) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(controller_status)
    } else {
        Err(status)
    }
}

pub fn i2c_master_reset_bus(ft_handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CMaster_ResetBus(ft_handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_slave_init(ft_handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CSlave_Init(ft_handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_slave_reset(ft_handle: FT_HANDLE) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CSlave_Reset(ft_handle) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_slave_get_address(ft_handle: FT_HANDLE) -> Result<u8, FT4222_STATUS> {
    let mut addr: u8 = 0;
    let status = unsafe { FT4222_I2CSlave_GetAddress(ft_handle, &mut addr) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(addr)
    } else {
        Err(status)
    }
}

pub fn i2c_slave_set_address(ft_handle: FT_HANDLE, addr: u8) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CSlave_SetAddress(ft_handle, addr) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_slave_get_rx_status(ft_handle: FT_HANDLE) -> Result<u16, FT4222_STATUS> {
    let mut rx_size: u16 = 0;
    let status = unsafe { FT4222_I2CSlave_GetRxStatus(ft_handle, &mut rx_size) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(rx_size)
    } else {
        Err(status)
    }
}

pub fn i2c_slave_read(ft_handle: FT_HANDLE, buffer: &mut [u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_I2CSlave_Read(
            ft_handle,
            buffer.as_mut_ptr(),
            buffer.len() as u16,
            &mut size_transferred,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn i2c_slave_write(ft_handle: FT_HANDLE, buffer: &[u8]) -> Result<usize, FT4222_STATUS> {
    let mut size_transferred: u16 = 0;
    let status = unsafe {
        FT4222_I2CSlave_Write(
            ft_handle,
            buffer.as_ptr(),
            buffer.len() as u16,
            &mut size_transferred,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_transferred as usize)
    } else {
        Err(status)
    }
}

pub fn i2c_slave_set_clock_stretch(ft_handle: FT_HANDLE, enable: bool) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CSlave_SetClockStretch(ft_handle, enable as BOOL) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn i2c_slave_set_resp_word(ft_handle: FT_HANDLE, response_word: u8) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_I2CSlave_SetRespWord(ft_handle, response_word) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn gpio_init(handle: FT_HANDLE, gpio_dir: &[GPIO_Dir; 4]) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_GPIO_Init(handle, gpio_dir.as_ptr()) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn gpio_read(handle: FT_HANDLE, port: GPIO_Port) -> Result<bool, FT4222_STATUS> {
    let mut value: BOOL = 0;
    let status = unsafe { FT4222_GPIO_Read(handle, port, &mut value) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(value != 0)
    } else {
        Err(status)
    }
}

pub fn gpio_write(ft_handle: FT_HANDLE, port_num: GPIO_Port, value: bool) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_GPIO_Write(ft_handle, port_num, value as BOOL) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn gpio_set_input_trigger(ft_handle: FT_HANDLE, port_num: GPIO_Port, trigger: GPIO_Trigger) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_GPIO_SetInputTrigger(ft_handle, port_num, trigger) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}

pub fn gpio_get_trigger_status(ft_handle: FT_HANDLE, port_num: GPIO_Port) -> Result<u16, FT4222_STATUS> {
    let mut queue_size: u16 = 0;
    let status = unsafe { FT4222_GPIO_GetTriggerStatus(ft_handle, port_num, &mut queue_size) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(queue_size)
    } else {
        Err(status)
    }
}

pub fn gpio_read_trigger_queue(ft_handle: FT_HANDLE, port_num: GPIO_Port, events: &mut [GPIO_Trigger]) -> Result<usize, FT4222_STATUS> {
    let mut size_of_read: u16 = 0;
    let status = unsafe {
        FT4222_GPIO_ReadTriggerQueue(
            ft_handle,
            port_num,
            events.as_mut_ptr(),
            events.len() as u16,
            &mut size_of_read,
        )
    };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(size_of_read as usize)
    } else {
        Err(status)
    }
}

pub fn gpio_set_wave_form_mode(ft_handle: FT_HANDLE, enable: bool) -> Result<(), FT4222_STATUS> {
    let status = unsafe { FT4222_GPIO_SetWaveFormMode(ft_handle, enable as BOOL) };
    if status == FT4222_STATUS::FT4222_OK {
        Ok(())
    } else {
        Err(status)
    }
}
