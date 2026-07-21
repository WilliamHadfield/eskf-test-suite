embassy_hal_internal::peripherals_definition!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD2,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    CAN2,
    CEC,
    CRC,
    DAC1,
    DBGMCU,
    DCMI,
    DMA1,
    DMA2,
    FLASH,
    FMPI2C1,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    PWR,
    QUADSPI,
    MCO1,
    MCO2,
    RCC,
    RTC,
    SAI1,
    SDIO,
    SPDIFRX1,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TIM1,
    TIM10,
    TIM11,
    TIM12,
    TIM13,
    TIM14,
    TIM2,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TIM9,
    UART4,
    UART5,
    UID,
    USART1,
    USART2,
    USART3,
    USART6,
    USB_OTG_FS,
    USB_OTG_HS,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH0,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::peripherals_struct!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD2,
    PH0,
    PH1,
    ADC1,
    ADC123_COMMON,
    ADC2,
    ADC3,
    CAN1,
    CAN2,
    CEC,
    CRC,
    DAC1,
    DBGMCU,
    DCMI,
    DMA1,
    DMA2,
    FLASH,
    FMPI2C1,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    PWR,
    QUADSPI,
    MCO1,
    MCO2,
    RCC,
    RTC,
    SAI1,
    SDIO,
    SPDIFRX1,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TIM1,
    TIM10,
    TIM11,
    TIM12,
    TIM13,
    TIM14,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    TIM9,
    UART4,
    UART5,
    UID,
    USART1,
    USART2,
    USART3,
    USART6,
    USB_OTG_FS,
    USB_OTG_HS,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH0,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD,
    TAMP_STAMP,
    RTC_WKUP,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_STREAM0,
    DMA1_STREAM1,
    DMA1_STREAM2,
    DMA1_STREAM3,
    DMA1_STREAM4,
    DMA1_STREAM5,
    DMA1_STREAM6,
    ADC,
    CAN1_TX,
    CAN1_RX0,
    CAN1_RX1,
    CAN1_SCE,
    EXTI9_5,
    TIM1_BRK_TIM9,
    TIM1_UP_TIM10,
    TIM1_TRG_COM_TIM11,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    OTG_FS_WKUP,
    TIM8_BRK_TIM12,
    TIM8_UP_TIM13,
    TIM8_TRG_COM_TIM14,
    TIM8_CC,
    DMA1_STREAM7,
    FMC,
    SDIO,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6_DAC,
    TIM7,
    DMA2_STREAM0,
    DMA2_STREAM1,
    DMA2_STREAM2,
    DMA2_STREAM3,
    DMA2_STREAM4,
    CAN2_TX,
    CAN2_RX0,
    CAN2_RX1,
    CAN2_SCE,
    OTG_FS,
    DMA2_STREAM5,
    DMA2_STREAM6,
    DMA2_STREAM7,
    USART6,
    I2C3_EV,
    I2C3_ER,
    OTG_HS_EP1_OUT,
    OTG_HS_EP1_IN,
    OTG_HS_WKUP,
    OTG_HS,
    DCMI,
    FPU,
    SPI4,
    SAI1,
    SAI2,
    QUADSPI,
    CEC,
    SPDIF_RX,
    FMPI2C1_EV,
    FMPI2C1_ER,
);
#[cfg(feature = "rt")]
#[interrupt]
fn TIM2() {
    crate::time_driver::get_driver().on_interrupt();
}
pub const MAX_ERASE_SIZE: usize = 131072u32 as usize;
pub mod flash_regions {
    impl crate::flash::FlashBank {
        #[doc = r" Absolute base address."]
        pub fn base(&self) -> u32 {
            match self {
                crate::flash::FlashBank::Bank1 => 134217728u32,
                crate::flash::FlashBank::Bank2 => panic!("Bank 2 not present"),
                crate::flash::FlashBank::Otp => 536836096u32,
            }
        }
    }
    pub const BANK1_REGION1: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        offset: 0u32,
        size: 65536u32,
        erase_size: 16384u32,
        write_size: 4u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region1<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK1_REGION2: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        offset: 65536u32,
        size: 65536u32,
        erase_size: 65536u32,
        write_size: 4u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region2<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK1_REGION3: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        offset: 131072u32,
        size: 393216u32,
        erase_size: 131072u32,
        write_size: 4u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region3<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const OTP_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Otp,
        offset: 0u32,
        size: 528u32,
        erase_size: 0u32,
        write_size: 4u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct OTPRegion<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region1: Bank1Region1<'d, MODE>,
        pub bank1_region2: Bank1Region2<'d, MODE>,
        pub bank1_region3: Bank1Region3<'d, MODE>,
        pub otp_region: OTPRegion<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(p: embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>) -> Self {
            Self {
                bank1_region1: Bank1Region1(
                    &BANK1_REGION1,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank1_region2: Bank1Region2(
                    &BANK1_REGION2,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank1_region3: Bank1Region3(
                    &BANK1_REGION3,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                otp_region: OTPRegion(
                    &OTP_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 4usize] =
        [&BANK1_REGION1, &BANK1_REGION2, &BANK1_REGION3, &OTP_REGION];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (17u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (17u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (17u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 25u8)),
            (16u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 26u8)),
            (16u8, 26u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CEC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().cecsel() {
            crate::pac::rcc::vals::Cecsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "lse")
            },
            crate::pac::rcc::vals::Cecsel::HSI_DIV_488 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "hsi") / 488u32
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "CEC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (16u8, 27u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CEC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 12u8)),
            (12u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::DAC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 29u8)),
            (16u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DAC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DCMI {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DCMI" , "hclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DCMI" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((5u8, 0u8)),
            (13u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DCMI {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 21u8)),
            (12u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 22u8)),
            (12u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FMPI2C1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().fmpi2c1sel() {
            crate::pac::rcc::vals::Fmpi2csel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMPI2C1" , "pclk1")
            },
            crate::pac::rcc::vals::Fmpi2csel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMPI2C1" , "sys")
            },
            crate::pac::rcc::vals::Fmpi2csel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMPI2C1" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "FMPI2C1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMPI2C1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 24u8)),
            (16u8, 24u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FMPI2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 21u8)),
            (16u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 22u8)),
            (16u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 23u8)),
            (16u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::PWR {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 28u8)),
            (16u8, 28u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::PWR {}
impl crate::rcc::SealedRccPeripheral for peripherals::QUADSPI {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((6u8, 1u8)),
            (14u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::QUADSPI {}
impl crate::rcc::SealedRccPeripheral for peripherals::RTC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.bdcr().read().rtcsel() {
            crate::pac::rcc::vals::Rtcsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lse")
            },
            crate::pac::rcc::vals::Rtcsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lsi")
            },
            crate::pac::rcc::vals::Rtcsel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "RTC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (16u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Standby,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RTC {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 22u8)),
            (17u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SDIO {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr().read().sdiosel() {
            crate::pac::rcc::vals::Sdiosel::CLK48 => {
                match crate::pac::RCC.dckcfgr().read().clk48sel() {
                    crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDIO" , "pll1_q")
                    },
                    crate::pac::rcc::vals::Clk48sel::PLLSAI1_Q => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDIO" , "pllsai1_q")
                    },
                    #[allow(unreachable_patterns)]
                    _ => panic!(
                        "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                        "SDIO"
                    ),
                }
            }
            crate::pac::rcc::vals::Sdiosel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDIO" , "sys")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SDIO"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDIO" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 11u8)),
            (17u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SDIO {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPDIFRX1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr2().read().spdifrxsel() {
            crate::pac::rcc::vals::Spdifrxsel::PLL1_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "pll1_r")
            },
            crate::pac::rcc::vals::Spdifrxsel::PLLI2S1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . plli2s1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "plli2s1_p")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SPDIFRX1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 16u8)),
            (16u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPDIFRX1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 12u8)),
            (17u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 14u8)),
            (16u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 15u8)),
            (16u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SYSCFG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 14u8)),
            (17u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SYSCFG {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 0u8)),
            (17u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM10 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM10" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM10" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 17u8)),
            (17u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM10 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM11 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM11" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM11" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 18u8)),
            (17u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM11 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM12 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM12" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM12" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 6u8)),
            (16u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM12 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM13 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM13" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM13" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 7u8)),
            (16u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM13 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM14 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 8u8)),
            (16u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM14 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 0u8)),
            (16u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 1u8)),
            (16u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 2u8)),
            (16u8, 2u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 3u8)),
            (16u8, 3u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 4u8)),
            (16u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM7 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 5u8)),
            (16u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM8 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 1u8)),
            (17u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM8 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM9 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM9" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM9" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 16u8)),
            (17u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM9 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 19u8)),
            (16u8, 19u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 20u8)),
            (16u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 4u8)),
            (17u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 17u8)),
            (16u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 18u8)),
            (16u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((9u8, 5u8)),
            (17u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_FS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pll1_q")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pllsai1_q")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_FS"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((5u8, 7u8)),
            (13u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_FS {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_HS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.dckcfgr().read().clk48sel() {
            crate::pac::rcc::vals::Clk48sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "pll1_q")
            },
            crate::pac::rcc::vals::Clk48sel::PLLSAI1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pllsai1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "pllsai1_q")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_HS"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 29u8)),
            (12u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_HS {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((8u8, 11u8)),
            (16u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG {}
pub(crate) static mut REFCOUNTS: [u8; 0usize] = [];
pub mod mux {
    pub use crate::pac::rcc::vals::Cecsel;
    pub use crate::pac::rcc::vals::Clk48sel;
    pub use crate::pac::rcc::vals::Fmpi2csel;
    pub use crate::pac::rcc::vals::Rtcsel;
    pub use crate::pac::rcc::vals::Sdiosel;
    pub use crate::pac::rcc::vals::Spdifrxsel;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {
        pub rtcsel: Rtcsel,
        pub clk48sel: Clk48sel,
        pub sdiosel: Sdiosel,
        pub cecsel: Cecsel,
        pub fmpi2c1sel: Fmpi2csel,
        pub spdifrxsel: Spdifrxsel,
    }
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {
            crate::pac::RCC.bdcr().modify(|w| {
                w.set_rtcsel(self.rtcsel);
            });
            crate::pac::RCC.dckcfgr().modify(|w| {
                w.set_clk48sel(self.clk48sel);
                w.set_sdiosel(self.sdiosel);
            });
            crate::pac::RCC.dckcfgr2().modify(|w| {
                w.set_cecsel(self.cecsel);
                w.set_fmpi2c1sel(self.fmpi2c1sel);
                w.set_spdifrxsel(self.spdifrxsel);
            });
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub hclk1: crate::time::MaybeHertz,
    pub hclk2: crate::time::MaybeHertz,
    pub hclk3: crate::time::MaybeHertz,
    pub hse: crate::time::MaybeHertz,
    pub hsi: crate::time::MaybeHertz,
    pub lse: crate::time::MaybeHertz,
    pub lsi: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pclk2: crate::time::MaybeHertz,
    pub pclk2_tim: crate::time::MaybeHertz,
    pub pll1_q: crate::time::MaybeHertz,
    pub pll1_r: crate::time::MaybeHertz,
    pub plli2s1_p: crate::time::MaybeHertz,
    pub plli2s1_q: crate::time::MaybeHertz,
    pub plli2s1_r: crate::time::MaybeHertz,
    pub pllsai1_q: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
}
pub unsafe fn init_mdma() {}
pub unsafe fn init_dma() {
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma1en(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma2en(true));
}
pub unsafe fn init_bdma() {}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpioeen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiofen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiogen(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_gpiohen(true));
}
impl_adc_pin!(ADC1, PA0, 0u8);
impl_adc_pin!(ADC1, PA1, 1u8);
impl_adc_pin!(ADC1, PA2, 2u8);
impl_adc_pin!(ADC1, PA3, 3u8);
impl_adc_pin!(ADC1, PA4, 4u8);
impl_adc_pin!(ADC1, PA5, 5u8);
impl_adc_pin!(ADC1, PA6, 6u8);
impl_adc_pin!(ADC1, PA7, 7u8);
impl_adc_pin!(ADC1, PB0, 8u8);
impl_adc_pin!(ADC1, PB1, 9u8);
impl_adc_pin!(ADC1, PC0, 10u8);
impl_adc_pin!(ADC1, PC1, 11u8);
impl_adc_pin!(ADC1, PC2, 12u8);
impl_adc_pin!(ADC1, PC3, 13u8);
impl_adc_pin!(ADC1, PC4, 14u8);
impl_adc_pin!(ADC1, PC5, 15u8);
impl_adc_pin!(ADC2, PA0, 0u8);
impl_adc_pin!(ADC2, PA1, 1u8);
impl_adc_pin!(ADC2, PA2, 2u8);
impl_adc_pin!(ADC2, PA3, 3u8);
impl_adc_pin!(ADC2, PA4, 4u8);
impl_adc_pin!(ADC2, PA5, 5u8);
impl_adc_pin!(ADC2, PA6, 6u8);
impl_adc_pin!(ADC2, PA7, 7u8);
impl_adc_pin!(ADC2, PB0, 8u8);
impl_adc_pin!(ADC2, PB1, 9u8);
impl_adc_pin!(ADC2, PC0, 10u8);
impl_adc_pin!(ADC2, PC1, 11u8);
impl_adc_pin!(ADC2, PC2, 12u8);
impl_adc_pin!(ADC2, PC3, 13u8);
impl_adc_pin!(ADC2, PC4, 14u8);
impl_adc_pin!(ADC2, PC5, 15u8);
impl_adc_pin!(ADC3, PA0, 0u8);
impl_adc_pin!(ADC3, PA1, 1u8);
impl_adc_pin!(ADC3, PA2, 2u8);
impl_adc_pin!(ADC3, PA3, 3u8);
impl_adc_pin!(ADC3, PC0, 10u8);
impl_adc_pin!(ADC3, PC1, 11u8);
impl_adc_pin!(ADC3, PC2, 12u8);
impl_adc_pin!(ADC3, PC3, 13u8);
pin_trait_impl!(
    crate::can::RxPin,
    CAN1,
    PA11,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN1,
    PA12,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::RxPin,
    CAN1,
    PB8,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN1,
    PB9,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::RxPin,
    CAN2,
    PB12,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN2,
    PB13,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::RxPin,
    CAN2,
    PB5,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::can::TxPin,
    CAN2,
    PB6,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::dac::DacPin<Ch1>, DAC1, PA4, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch2>, DAC1, PA5, 0u8);
pin_trait_impl!(crate::dcmi::D1Pin, DCMI, PA10, 13u8);
pin_trait_impl!(crate::dcmi::HSyncPin, DCMI, PA4, 13u8);
pin_trait_impl!(crate::dcmi::PixClkPin, DCMI, PA6, 13u8);
pin_trait_impl!(crate::dcmi::D0Pin, DCMI, PA9, 13u8);
pin_trait_impl!(crate::dcmi::D10Pin, DCMI, PB5, 13u8);
pin_trait_impl!(crate::dcmi::D5Pin, DCMI, PB6, 13u8);
pin_trait_impl!(crate::dcmi::VSyncPin, DCMI, PB7, 13u8);
pin_trait_impl!(crate::dcmi::D6Pin, DCMI, PB8, 13u8);
pin_trait_impl!(crate::dcmi::D7Pin, DCMI, PB9, 13u8);
pin_trait_impl!(crate::dcmi::D8Pin, DCMI, PC10, 13u8);
pin_trait_impl!(crate::dcmi::D4Pin, DCMI, PC11, 13u8);
pin_trait_impl!(crate::dcmi::D9Pin, DCMI, PC12, 13u8);
pin_trait_impl!(crate::dcmi::D0Pin, DCMI, PC6, 13u8);
pin_trait_impl!(crate::dcmi::D1Pin, DCMI, PC7, 13u8);
pin_trait_impl!(crate::dcmi::D2Pin, DCMI, PC8, 13u8);
pin_trait_impl!(crate::dcmi::D3Pin, DCMI, PC9, 13u8);
pin_trait_impl!(crate::dcmi::D11Pin, DCMI, PD2, 13u8);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB6,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB7,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB8,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB9,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C2,
    PB10,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C2,
    PB3,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C2,
    PC12,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C3,
    PA8,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C3,
    PB4,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C3,
    PC9,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PA1, 9u8);
pin_trait_impl!(crate::qspi::SckPin, QUADSPI, PB2, 9u8);
pin_trait_impl!(crate::qspi::BK1NSSPin, QUADSPI, PB6, 10u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PC10, 9u8);
pin_trait_impl!(crate::qspi::BK2NSSPin, QUADSPI, PC11, 9u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PC9, 9u8);
pin_trait_impl!(crate::rcc::McoPin, MCO1, PA8, 0u8);
pin_trait_impl!(crate::rcc::McoPin, MCO2, PC9, 0u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI1, PA3, 6u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI1, PA9, 6u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI1, PB10, 6u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI1, PB12, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PB2, 6u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI1, PB9, 6u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI1, PC0, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PC1, 6u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDIO, PB0, 12u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDIO, PB1, 12u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDIO, PB2, 12u8);
pin_trait_impl!(crate::sdmmc::D4Pin, SDIO, PB8, 12u8);
pin_trait_impl!(crate::sdmmc::D5Pin, SDIO, PB9, 12u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDIO, PC10, 12u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDIO, PC11, 12u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDIO, PC12, 12u8);
pin_trait_impl!(crate::sdmmc::D6Pin, SDIO, PC6, 12u8);
pin_trait_impl!(crate::sdmmc::D7Pin, SDIO, PC7, 12u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDIO, PC8, 12u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDIO, PC9, 12u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDIO, PD2, 12u8);
impl_spdifrx_pin!(SPDIFRX1, PB7, 8u8, 0u8);
impl_spdifrx_pin!(SPDIFRX1, PC4, 8u8, 2u8);
impl_spdifrx_pin!(SPDIFRX1, PC5, 8u8, 3u8);
impl_spdifrx_pin!(SPDIFRX1, PC7, 7u8, 1u8);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI1,
    PA15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI1,
    PA4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI1,
    PA5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PA5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PA6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI1,
    PA7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PA7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI1,
    PB3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PB3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PB4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI1,
    PB5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PB5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MckPin,
    SPI1,
    PC4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
impl_i2_ext_instance!(SPI2, I2S2);
pin_trait_impl!(
    crate::spi::MckPin,
    SPI2,
    PA6,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PA9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PA9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PB10,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB10,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI2,
    PB12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PB13,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB13,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PB14,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PB15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PB15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI2,
    PB4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI2,
    PB9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PC1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PC1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PC2,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PC3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PC3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MckPin,
    SPI2,
    PC6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PC7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PC7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
impl_i2_ext_instance!(SPI3, I2S3);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI3,
    PA15,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA15,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI3,
    PA4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PB0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PB0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PB2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PB2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI3,
    PB3,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PB3,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PB4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PB5,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PB5,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PC1,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PC1,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI3,
    PC10,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PC10,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PC11,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PC12,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PC12,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MckPin,
    SPI3,
    PC7,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM1,
    PA10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM1,
    PA11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM1,
    PA12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PA6,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PA7,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM1,
    PA8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM1,
    PA9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PB12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PB13,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB14,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM10,
    PB8,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM11,
    PB9,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM12,
    PB14,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM12,
    PB15,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM13,
    PA6,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM14,
    PA7,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PA1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PA2,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PA3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA5,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA5,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PB10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PB2,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PB3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PB8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PB8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PB9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PA6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PA7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PB0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PB1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PB4,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PB5,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PC6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PC7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PC8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PC9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM3,
    PD2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM4,
    PB6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM4,
    PB7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM4,
    PB8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM4,
    PB9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM5,
    PA0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM5,
    PA1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM5,
    PA2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM5,
    PA3,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM8,
    PA0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA5,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM8,
    PA6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB1,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB14,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB15,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM8,
    PC6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM8,
    PC7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM8,
    PC8,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM8,
    PC9,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM9,
    PA2,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM9,
    PA3,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PA0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PA1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART4,
    PA15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART4,
    PB0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PC10,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PC11,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART5,
    PC12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART5,
    PC8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART5,
    PC9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART5,
    PD2,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PA10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART1,
    PA11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART1,
    PA12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART1,
    PA8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PA9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PB6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PB7,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART2,
    PA0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART2,
    PA1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART2,
    PA2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART2,
    PA3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART2,
    PA4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PB10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PB12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART3,
    PB13,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PB14,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PC10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PC11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PC12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PC5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART6,
    PC6,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART6,
    PC7,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART6,
    PC8,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_FS, PA11, 10u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_FS, PA12, 10u8);
pin_trait_impl!(crate::usb::UlpiD0Pin, USB_OTG_HS, PA3, 10u8);
pin_trait_impl!(crate::usb::UlpiClkPin, USB_OTG_HS, PA5, 10u8);
pin_trait_impl!(crate::usb::UlpiD1Pin, USB_OTG_HS, PB0, 10u8);
pin_trait_impl!(crate::usb::UlpiD2Pin, USB_OTG_HS, PB1, 10u8);
pin_trait_impl!(crate::usb::UlpiD3Pin, USB_OTG_HS, PB10, 10u8);
pin_trait_impl!(crate::usb::UlpiD5Pin, USB_OTG_HS, PB12, 10u8);
pin_trait_impl!(crate::usb::UlpiD6Pin, USB_OTG_HS, PB13, 10u8);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_HS, PB14, 12u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_HS, PB15, 12u8);
pin_trait_impl!(crate::usb::UlpiD4Pin, USB_OTG_HS, PB2, 10u8);
pin_trait_impl!(crate::usb::UlpiD7Pin, USB_OTG_HS, PB5, 10u8);
pin_trait_impl!(crate::usb::UlpiStpPin, USB_OTG_HS, PC0, 10u8);
pin_trait_impl!(crate::usb::UlpiDirPin, USB_OTG_HS, PC2, 10u8);
pin_trait_impl!(crate::usb::UlpiNxtPin, USB_OTG_HS, PC3, 10u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM1_CH1, 0u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM1_CH2, 1u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM1_CH3, 2u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM2_CH2, 3u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM2_CH3, 4u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM2_CH4, 5u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM2_TRGO, 6u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM3_CH1, 7u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM3_TRGO, 8u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM4_CH4, 9u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM5_CH1, 10u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM5_CH2, 11u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM5_CH3, 12u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM8_CH1, 13u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, TIM8_TRGO, 14u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC1, EXTI11_TRG, 15u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM1_CH4, 0u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM1_TRGO, 1u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM2_CH1, 2u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM2_TRGO, 3u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM3_CH2, 4u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM3_CH4, 5u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM4_CH1, 6u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM4_CH2, 7u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM4_CH3, 8u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM4_TRGO, 9u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM5_CH4, 10u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM5_TRGO, 11u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM8_CH2, 12u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM8_CH3, 13u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, TIM8_CH4, 14u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC1, EXTI15_TRG, 15u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH0, 0u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH4, 0u8, {});
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM1_CH1, 0u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM1_CH2, 1u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM1_CH3, 2u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM2_CH2, 3u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM2_CH3, 4u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM2_CH4, 5u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM2_TRGO, 6u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM3_CH1, 7u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM3_TRGO, 8u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM4_CH4, 9u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM5_CH1, 10u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM5_CH2, 11u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM5_CH3, 12u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM8_CH1, 13u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, TIM8_TRGO, 14u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC2, EXTI11_TRG, 15u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM1_CH4, 0u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM1_TRGO, 1u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM2_CH1, 2u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM2_TRGO, 3u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM3_CH2, 4u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM3_CH4, 5u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM4_CH1, 6u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM4_CH2, 7u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM4_CH3, 8u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM4_TRGO, 9u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM5_CH4, 10u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM5_TRGO, 11u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM8_CH2, 12u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM8_CH3, 13u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, TIM8_CH4, 14u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC2, EXTI15_TRG, 15u8);
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH2, 1u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH3, 1u8, {});
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM1_CH1, 0u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM1_CH2, 1u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM1_CH3, 2u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM2_CH2, 3u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM2_CH3, 4u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM2_CH4, 5u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM2_TRGO, 6u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM3_CH1, 7u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM3_TRGO, 8u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM4_CH4, 9u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM5_CH1, 10u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM5_CH2, 11u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM5_CH3, 12u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM8_CH1, 13u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, TIM8_TRGO, 14u8);
trigger_trait_impl!(crate::adc::RegularTrigger, ADC3, EXTI11_TRG, 15u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM1_CH4, 0u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM1_TRGO, 1u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM2_CH1, 2u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM2_TRGO, 3u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM3_CH2, 4u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM3_CH4, 5u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM4_CH1, 6u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM4_CH2, 7u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM4_CH3, 8u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM4_TRGO, 9u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM5_CH4, 10u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM5_TRGO, 11u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM8_CH2, 12u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM8_CH3, 13u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, TIM8_CH4, 14u8);
trigger_trait_impl!(crate::adc::InjectedTrigger, ADC3, EXTI15_TRG, 15u8);
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH0, 2u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH1, 2u8, {});
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM6_TRGO, 0u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM8_TRGO, 1u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM7_TRGO, 2u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM5_TRGO, 3u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM2_TRGO, 4u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM4_TRGO, 5u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, EXTI9_TRG, 6u8);
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH5, 7u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH6, 7u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH1, 1u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH7, 1u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH0, 1u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH5, 1u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH6, 1u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH7, 1u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH2, 7u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH3, 7u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH7, 7u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH1, 1u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH2, 3u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH4, 3u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, DMA2_CH7, 3u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH1, 0u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH3, 0u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH4, 1u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH5, 0u8, {});
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDIO, DMA2_CH3, 4u8, {});
dma_trait_impl!(crate::sdmmc::SdmmcDma, SDIO, DMA2_CH6, 4u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH1, 0u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH6, 0u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH0, 3u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH2, 3u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH3, 3u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH5, 3u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH3, 0u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH4, 0u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH0, 0u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH2, 0u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH5, 0u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH7, 0u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH1, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH2, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH3, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH4, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH5, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH6, 0u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH6, 0u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH6, 0u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH1, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH1, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH5, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH6, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH6, 3u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH7, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH7, 3u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH2, 5u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH2, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH4, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH5, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH7, 5u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH0, 2u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH3, 2u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH6, 2u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH7, 2u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH0, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH0, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH1, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH2, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH3, 6u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH4, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH6, 6u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH1, 7u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH2, 1u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH4, 1u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH1, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH2, 0u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH2, 0u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH2, 0u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH3, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH4, 7u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH7, 7u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH2, 4u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH4, 4u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH0, 4u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH7, 4u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH2, 4u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH5, 4u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH7, 4u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH5, 4u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH6, 4u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH1, 4u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH3, 4u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH4, 7u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH1, 5u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH2, 5u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH6, 5u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH7, 5u8, {});
pub mod triggers {
    #[allow(non_camel_case_types)]
    pub struct EXTI11_TRG;
    #[allow(non_camel_case_types)]
    pub struct EXTI15_TRG;
    #[allow(non_camel_case_types)]
    pub struct EXTI9_TRG;
    #[allow(non_camel_case_types)]
    pub struct TIM1_CH1;
    #[allow(non_camel_case_types)]
    pub struct TIM1_CH2;
    #[allow(non_camel_case_types)]
    pub struct TIM1_CH3;
    #[allow(non_camel_case_types)]
    pub struct TIM1_CH4;
    #[allow(non_camel_case_types)]
    pub struct TIM1_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM2_CH1;
    #[allow(non_camel_case_types)]
    pub struct TIM2_CH2;
    #[allow(non_camel_case_types)]
    pub struct TIM2_CH3;
    #[allow(non_camel_case_types)]
    pub struct TIM2_CH4;
    #[allow(non_camel_case_types)]
    pub struct TIM2_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM3_CH1;
    #[allow(non_camel_case_types)]
    pub struct TIM3_CH2;
    #[allow(non_camel_case_types)]
    pub struct TIM3_CH4;
    #[allow(non_camel_case_types)]
    pub struct TIM3_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM4_CH1;
    #[allow(non_camel_case_types)]
    pub struct TIM4_CH2;
    #[allow(non_camel_case_types)]
    pub struct TIM4_CH3;
    #[allow(non_camel_case_types)]
    pub struct TIM4_CH4;
    #[allow(non_camel_case_types)]
    pub struct TIM4_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM5_CH1;
    #[allow(non_camel_case_types)]
    pub struct TIM5_CH2;
    #[allow(non_camel_case_types)]
    pub struct TIM5_CH3;
    #[allow(non_camel_case_types)]
    pub struct TIM5_CH4;
    #[allow(non_camel_case_types)]
    pub struct TIM5_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM6_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM7_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM8_CH1;
    #[allow(non_camel_case_types)]
    pub struct TIM8_CH2;
    #[allow(non_camel_case_types)]
    pub struct TIM8_CH3;
    #[allow(non_camel_case_types)]
    pub struct TIM8_CH4;
    #[allow(non_camel_case_types)]
    pub struct TIM8_TRGO;
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Hpre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hpre::DIV1 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hpre::DIV1 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Mcopre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Mcopre::DIV1 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => 2u32,
            crate::pac::rcc::vals::Mcopre::DIV3 => 3u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => 4u32,
            crate::pac::rcc::vals::Mcopre::DIV5 => 5u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Mcopre::DIV1 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV3 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV5 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllm {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllm::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllm::DIV3 => 3u32,
            crate::pac::rcc::vals::Pllm::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllm::DIV5 => 5u32,
            crate::pac::rcc::vals::Pllm::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllm::DIV7 => 7u32,
            crate::pac::rcc::vals::Pllm::DIV8 => 8u32,
            crate::pac::rcc::vals::Pllm::DIV9 => 9u32,
            crate::pac::rcc::vals::Pllm::DIV10 => 10u32,
            crate::pac::rcc::vals::Pllm::DIV11 => 11u32,
            crate::pac::rcc::vals::Pllm::DIV12 => 12u32,
            crate::pac::rcc::vals::Pllm::DIV13 => 13u32,
            crate::pac::rcc::vals::Pllm::DIV14 => 14u32,
            crate::pac::rcc::vals::Pllm::DIV15 => 15u32,
            crate::pac::rcc::vals::Pllm::DIV16 => 16u32,
            crate::pac::rcc::vals::Pllm::DIV17 => 17u32,
            crate::pac::rcc::vals::Pllm::DIV18 => 18u32,
            crate::pac::rcc::vals::Pllm::DIV19 => 19u32,
            crate::pac::rcc::vals::Pllm::DIV20 => 20u32,
            crate::pac::rcc::vals::Pllm::DIV21 => 21u32,
            crate::pac::rcc::vals::Pllm::DIV22 => 22u32,
            crate::pac::rcc::vals::Pllm::DIV23 => 23u32,
            crate::pac::rcc::vals::Pllm::DIV24 => 24u32,
            crate::pac::rcc::vals::Pllm::DIV25 => 25u32,
            crate::pac::rcc::vals::Pllm::DIV26 => 26u32,
            crate::pac::rcc::vals::Pllm::DIV27 => 27u32,
            crate::pac::rcc::vals::Pllm::DIV28 => 28u32,
            crate::pac::rcc::vals::Pllm::DIV29 => 29u32,
            crate::pac::rcc::vals::Pllm::DIV30 => 30u32,
            crate::pac::rcc::vals::Pllm::DIV31 => 31u32,
            crate::pac::rcc::vals::Pllm::DIV32 => 32u32,
            crate::pac::rcc::vals::Pllm::DIV33 => 33u32,
            crate::pac::rcc::vals::Pllm::DIV34 => 34u32,
            crate::pac::rcc::vals::Pllm::DIV35 => 35u32,
            crate::pac::rcc::vals::Pllm::DIV36 => 36u32,
            crate::pac::rcc::vals::Pllm::DIV37 => 37u32,
            crate::pac::rcc::vals::Pllm::DIV38 => 38u32,
            crate::pac::rcc::vals::Pllm::DIV39 => 39u32,
            crate::pac::rcc::vals::Pllm::DIV40 => 40u32,
            crate::pac::rcc::vals::Pllm::DIV41 => 41u32,
            crate::pac::rcc::vals::Pllm::DIV42 => 42u32,
            crate::pac::rcc::vals::Pllm::DIV43 => 43u32,
            crate::pac::rcc::vals::Pllm::DIV44 => 44u32,
            crate::pac::rcc::vals::Pllm::DIV45 => 45u32,
            crate::pac::rcc::vals::Pllm::DIV46 => 46u32,
            crate::pac::rcc::vals::Pllm::DIV47 => 47u32,
            crate::pac::rcc::vals::Pllm::DIV48 => 48u32,
            crate::pac::rcc::vals::Pllm::DIV49 => 49u32,
            crate::pac::rcc::vals::Pllm::DIV50 => 50u32,
            crate::pac::rcc::vals::Pllm::DIV51 => 51u32,
            crate::pac::rcc::vals::Pllm::DIV52 => 52u32,
            crate::pac::rcc::vals::Pllm::DIV53 => 53u32,
            crate::pac::rcc::vals::Pllm::DIV54 => 54u32,
            crate::pac::rcc::vals::Pllm::DIV55 => 55u32,
            crate::pac::rcc::vals::Pllm::DIV56 => 56u32,
            crate::pac::rcc::vals::Pllm::DIV57 => 57u32,
            crate::pac::rcc::vals::Pllm::DIV58 => 58u32,
            crate::pac::rcc::vals::Pllm::DIV59 => 59u32,
            crate::pac::rcc::vals::Pllm::DIV60 => 60u32,
            crate::pac::rcc::vals::Pllm::DIV61 => 61u32,
            crate::pac::rcc::vals::Pllm::DIV62 => 62u32,
            crate::pac::rcc::vals::Pllm::DIV63 => 63u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllm::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV3 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV5 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV7 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV8 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV9 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV10 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV11 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV12 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV13 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV14 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV15 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV16 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV17 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV18 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV19 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV20 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV21 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV22 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV23 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV24 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV25 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV26 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV27 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV28 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV29 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV30 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV31 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV32 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV33 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV34 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV35 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV36 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV37 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV38 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV39 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV40 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV41 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV42 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV43 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV44 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV45 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV46 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV47 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV48 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV49 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV50 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV51 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV52 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV53 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV54 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV55 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV56 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV57 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV58 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV59 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV60 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV61 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV62 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV63 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Plln {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plln::MUL50 => 50u32,
            crate::pac::rcc::vals::Plln::MUL51 => 51u32,
            crate::pac::rcc::vals::Plln::MUL52 => 52u32,
            crate::pac::rcc::vals::Plln::MUL53 => 53u32,
            crate::pac::rcc::vals::Plln::MUL54 => 54u32,
            crate::pac::rcc::vals::Plln::MUL55 => 55u32,
            crate::pac::rcc::vals::Plln::MUL56 => 56u32,
            crate::pac::rcc::vals::Plln::MUL57 => 57u32,
            crate::pac::rcc::vals::Plln::MUL58 => 58u32,
            crate::pac::rcc::vals::Plln::MUL59 => 59u32,
            crate::pac::rcc::vals::Plln::MUL60 => 60u32,
            crate::pac::rcc::vals::Plln::MUL61 => 61u32,
            crate::pac::rcc::vals::Plln::MUL62 => 62u32,
            crate::pac::rcc::vals::Plln::MUL63 => 63u32,
            crate::pac::rcc::vals::Plln::MUL64 => 64u32,
            crate::pac::rcc::vals::Plln::MUL65 => 65u32,
            crate::pac::rcc::vals::Plln::MUL66 => 66u32,
            crate::pac::rcc::vals::Plln::MUL67 => 67u32,
            crate::pac::rcc::vals::Plln::MUL68 => 68u32,
            crate::pac::rcc::vals::Plln::MUL69 => 69u32,
            crate::pac::rcc::vals::Plln::MUL70 => 70u32,
            crate::pac::rcc::vals::Plln::MUL71 => 71u32,
            crate::pac::rcc::vals::Plln::MUL72 => 72u32,
            crate::pac::rcc::vals::Plln::MUL73 => 73u32,
            crate::pac::rcc::vals::Plln::MUL74 => 74u32,
            crate::pac::rcc::vals::Plln::MUL75 => 75u32,
            crate::pac::rcc::vals::Plln::MUL76 => 76u32,
            crate::pac::rcc::vals::Plln::MUL77 => 77u32,
            crate::pac::rcc::vals::Plln::MUL78 => 78u32,
            crate::pac::rcc::vals::Plln::MUL79 => 79u32,
            crate::pac::rcc::vals::Plln::MUL80 => 80u32,
            crate::pac::rcc::vals::Plln::MUL81 => 81u32,
            crate::pac::rcc::vals::Plln::MUL82 => 82u32,
            crate::pac::rcc::vals::Plln::MUL83 => 83u32,
            crate::pac::rcc::vals::Plln::MUL84 => 84u32,
            crate::pac::rcc::vals::Plln::MUL85 => 85u32,
            crate::pac::rcc::vals::Plln::MUL86 => 86u32,
            crate::pac::rcc::vals::Plln::MUL87 => 87u32,
            crate::pac::rcc::vals::Plln::MUL88 => 88u32,
            crate::pac::rcc::vals::Plln::MUL89 => 89u32,
            crate::pac::rcc::vals::Plln::MUL90 => 90u32,
            crate::pac::rcc::vals::Plln::MUL91 => 91u32,
            crate::pac::rcc::vals::Plln::MUL92 => 92u32,
            crate::pac::rcc::vals::Plln::MUL93 => 93u32,
            crate::pac::rcc::vals::Plln::MUL94 => 94u32,
            crate::pac::rcc::vals::Plln::MUL95 => 95u32,
            crate::pac::rcc::vals::Plln::MUL96 => 96u32,
            crate::pac::rcc::vals::Plln::MUL97 => 97u32,
            crate::pac::rcc::vals::Plln::MUL98 => 98u32,
            crate::pac::rcc::vals::Plln::MUL99 => 99u32,
            crate::pac::rcc::vals::Plln::MUL100 => 100u32,
            crate::pac::rcc::vals::Plln::MUL101 => 101u32,
            crate::pac::rcc::vals::Plln::MUL102 => 102u32,
            crate::pac::rcc::vals::Plln::MUL103 => 103u32,
            crate::pac::rcc::vals::Plln::MUL104 => 104u32,
            crate::pac::rcc::vals::Plln::MUL105 => 105u32,
            crate::pac::rcc::vals::Plln::MUL106 => 106u32,
            crate::pac::rcc::vals::Plln::MUL107 => 107u32,
            crate::pac::rcc::vals::Plln::MUL108 => 108u32,
            crate::pac::rcc::vals::Plln::MUL109 => 109u32,
            crate::pac::rcc::vals::Plln::MUL110 => 110u32,
            crate::pac::rcc::vals::Plln::MUL111 => 111u32,
            crate::pac::rcc::vals::Plln::MUL112 => 112u32,
            crate::pac::rcc::vals::Plln::MUL113 => 113u32,
            crate::pac::rcc::vals::Plln::MUL114 => 114u32,
            crate::pac::rcc::vals::Plln::MUL115 => 115u32,
            crate::pac::rcc::vals::Plln::MUL116 => 116u32,
            crate::pac::rcc::vals::Plln::MUL117 => 117u32,
            crate::pac::rcc::vals::Plln::MUL118 => 118u32,
            crate::pac::rcc::vals::Plln::MUL119 => 119u32,
            crate::pac::rcc::vals::Plln::MUL120 => 120u32,
            crate::pac::rcc::vals::Plln::MUL121 => 121u32,
            crate::pac::rcc::vals::Plln::MUL122 => 122u32,
            crate::pac::rcc::vals::Plln::MUL123 => 123u32,
            crate::pac::rcc::vals::Plln::MUL124 => 124u32,
            crate::pac::rcc::vals::Plln::MUL125 => 125u32,
            crate::pac::rcc::vals::Plln::MUL126 => 126u32,
            crate::pac::rcc::vals::Plln::MUL127 => 127u32,
            crate::pac::rcc::vals::Plln::MUL128 => 128u32,
            crate::pac::rcc::vals::Plln::MUL129 => 129u32,
            crate::pac::rcc::vals::Plln::MUL130 => 130u32,
            crate::pac::rcc::vals::Plln::MUL131 => 131u32,
            crate::pac::rcc::vals::Plln::MUL132 => 132u32,
            crate::pac::rcc::vals::Plln::MUL133 => 133u32,
            crate::pac::rcc::vals::Plln::MUL134 => 134u32,
            crate::pac::rcc::vals::Plln::MUL135 => 135u32,
            crate::pac::rcc::vals::Plln::MUL136 => 136u32,
            crate::pac::rcc::vals::Plln::MUL137 => 137u32,
            crate::pac::rcc::vals::Plln::MUL138 => 138u32,
            crate::pac::rcc::vals::Plln::MUL139 => 139u32,
            crate::pac::rcc::vals::Plln::MUL140 => 140u32,
            crate::pac::rcc::vals::Plln::MUL141 => 141u32,
            crate::pac::rcc::vals::Plln::MUL142 => 142u32,
            crate::pac::rcc::vals::Plln::MUL143 => 143u32,
            crate::pac::rcc::vals::Plln::MUL144 => 144u32,
            crate::pac::rcc::vals::Plln::MUL145 => 145u32,
            crate::pac::rcc::vals::Plln::MUL146 => 146u32,
            crate::pac::rcc::vals::Plln::MUL147 => 147u32,
            crate::pac::rcc::vals::Plln::MUL148 => 148u32,
            crate::pac::rcc::vals::Plln::MUL149 => 149u32,
            crate::pac::rcc::vals::Plln::MUL150 => 150u32,
            crate::pac::rcc::vals::Plln::MUL151 => 151u32,
            crate::pac::rcc::vals::Plln::MUL152 => 152u32,
            crate::pac::rcc::vals::Plln::MUL153 => 153u32,
            crate::pac::rcc::vals::Plln::MUL154 => 154u32,
            crate::pac::rcc::vals::Plln::MUL155 => 155u32,
            crate::pac::rcc::vals::Plln::MUL156 => 156u32,
            crate::pac::rcc::vals::Plln::MUL157 => 157u32,
            crate::pac::rcc::vals::Plln::MUL158 => 158u32,
            crate::pac::rcc::vals::Plln::MUL159 => 159u32,
            crate::pac::rcc::vals::Plln::MUL160 => 160u32,
            crate::pac::rcc::vals::Plln::MUL161 => 161u32,
            crate::pac::rcc::vals::Plln::MUL162 => 162u32,
            crate::pac::rcc::vals::Plln::MUL163 => 163u32,
            crate::pac::rcc::vals::Plln::MUL164 => 164u32,
            crate::pac::rcc::vals::Plln::MUL165 => 165u32,
            crate::pac::rcc::vals::Plln::MUL166 => 166u32,
            crate::pac::rcc::vals::Plln::MUL167 => 167u32,
            crate::pac::rcc::vals::Plln::MUL168 => 168u32,
            crate::pac::rcc::vals::Plln::MUL169 => 169u32,
            crate::pac::rcc::vals::Plln::MUL170 => 170u32,
            crate::pac::rcc::vals::Plln::MUL171 => 171u32,
            crate::pac::rcc::vals::Plln::MUL172 => 172u32,
            crate::pac::rcc::vals::Plln::MUL173 => 173u32,
            crate::pac::rcc::vals::Plln::MUL174 => 174u32,
            crate::pac::rcc::vals::Plln::MUL175 => 175u32,
            crate::pac::rcc::vals::Plln::MUL176 => 176u32,
            crate::pac::rcc::vals::Plln::MUL177 => 177u32,
            crate::pac::rcc::vals::Plln::MUL178 => 178u32,
            crate::pac::rcc::vals::Plln::MUL179 => 179u32,
            crate::pac::rcc::vals::Plln::MUL180 => 180u32,
            crate::pac::rcc::vals::Plln::MUL181 => 181u32,
            crate::pac::rcc::vals::Plln::MUL182 => 182u32,
            crate::pac::rcc::vals::Plln::MUL183 => 183u32,
            crate::pac::rcc::vals::Plln::MUL184 => 184u32,
            crate::pac::rcc::vals::Plln::MUL185 => 185u32,
            crate::pac::rcc::vals::Plln::MUL186 => 186u32,
            crate::pac::rcc::vals::Plln::MUL187 => 187u32,
            crate::pac::rcc::vals::Plln::MUL188 => 188u32,
            crate::pac::rcc::vals::Plln::MUL189 => 189u32,
            crate::pac::rcc::vals::Plln::MUL190 => 190u32,
            crate::pac::rcc::vals::Plln::MUL191 => 191u32,
            crate::pac::rcc::vals::Plln::MUL192 => 192u32,
            crate::pac::rcc::vals::Plln::MUL193 => 193u32,
            crate::pac::rcc::vals::Plln::MUL194 => 194u32,
            crate::pac::rcc::vals::Plln::MUL195 => 195u32,
            crate::pac::rcc::vals::Plln::MUL196 => 196u32,
            crate::pac::rcc::vals::Plln::MUL197 => 197u32,
            crate::pac::rcc::vals::Plln::MUL198 => 198u32,
            crate::pac::rcc::vals::Plln::MUL199 => 199u32,
            crate::pac::rcc::vals::Plln::MUL200 => 200u32,
            crate::pac::rcc::vals::Plln::MUL201 => 201u32,
            crate::pac::rcc::vals::Plln::MUL202 => 202u32,
            crate::pac::rcc::vals::Plln::MUL203 => 203u32,
            crate::pac::rcc::vals::Plln::MUL204 => 204u32,
            crate::pac::rcc::vals::Plln::MUL205 => 205u32,
            crate::pac::rcc::vals::Plln::MUL206 => 206u32,
            crate::pac::rcc::vals::Plln::MUL207 => 207u32,
            crate::pac::rcc::vals::Plln::MUL208 => 208u32,
            crate::pac::rcc::vals::Plln::MUL209 => 209u32,
            crate::pac::rcc::vals::Plln::MUL210 => 210u32,
            crate::pac::rcc::vals::Plln::MUL211 => 211u32,
            crate::pac::rcc::vals::Plln::MUL212 => 212u32,
            crate::pac::rcc::vals::Plln::MUL213 => 213u32,
            crate::pac::rcc::vals::Plln::MUL214 => 214u32,
            crate::pac::rcc::vals::Plln::MUL215 => 215u32,
            crate::pac::rcc::vals::Plln::MUL216 => 216u32,
            crate::pac::rcc::vals::Plln::MUL217 => 217u32,
            crate::pac::rcc::vals::Plln::MUL218 => 218u32,
            crate::pac::rcc::vals::Plln::MUL219 => 219u32,
            crate::pac::rcc::vals::Plln::MUL220 => 220u32,
            crate::pac::rcc::vals::Plln::MUL221 => 221u32,
            crate::pac::rcc::vals::Plln::MUL222 => 222u32,
            crate::pac::rcc::vals::Plln::MUL223 => 223u32,
            crate::pac::rcc::vals::Plln::MUL224 => 224u32,
            crate::pac::rcc::vals::Plln::MUL225 => 225u32,
            crate::pac::rcc::vals::Plln::MUL226 => 226u32,
            crate::pac::rcc::vals::Plln::MUL227 => 227u32,
            crate::pac::rcc::vals::Plln::MUL228 => 228u32,
            crate::pac::rcc::vals::Plln::MUL229 => 229u32,
            crate::pac::rcc::vals::Plln::MUL230 => 230u32,
            crate::pac::rcc::vals::Plln::MUL231 => 231u32,
            crate::pac::rcc::vals::Plln::MUL232 => 232u32,
            crate::pac::rcc::vals::Plln::MUL233 => 233u32,
            crate::pac::rcc::vals::Plln::MUL234 => 234u32,
            crate::pac::rcc::vals::Plln::MUL235 => 235u32,
            crate::pac::rcc::vals::Plln::MUL236 => 236u32,
            crate::pac::rcc::vals::Plln::MUL237 => 237u32,
            crate::pac::rcc::vals::Plln::MUL238 => 238u32,
            crate::pac::rcc::vals::Plln::MUL239 => 239u32,
            crate::pac::rcc::vals::Plln::MUL240 => 240u32,
            crate::pac::rcc::vals::Plln::MUL241 => 241u32,
            crate::pac::rcc::vals::Plln::MUL242 => 242u32,
            crate::pac::rcc::vals::Plln::MUL243 => 243u32,
            crate::pac::rcc::vals::Plln::MUL244 => 244u32,
            crate::pac::rcc::vals::Plln::MUL245 => 245u32,
            crate::pac::rcc::vals::Plln::MUL246 => 246u32,
            crate::pac::rcc::vals::Plln::MUL247 => 247u32,
            crate::pac::rcc::vals::Plln::MUL248 => 248u32,
            crate::pac::rcc::vals::Plln::MUL249 => 249u32,
            crate::pac::rcc::vals::Plln::MUL250 => 250u32,
            crate::pac::rcc::vals::Plln::MUL251 => 251u32,
            crate::pac::rcc::vals::Plln::MUL252 => 252u32,
            crate::pac::rcc::vals::Plln::MUL253 => 253u32,
            crate::pac::rcc::vals::Plln::MUL254 => 254u32,
            crate::pac::rcc::vals::Plln::MUL255 => 255u32,
            crate::pac::rcc::vals::Plln::MUL256 => 256u32,
            crate::pac::rcc::vals::Plln::MUL257 => 257u32,
            crate::pac::rcc::vals::Plln::MUL258 => 258u32,
            crate::pac::rcc::vals::Plln::MUL259 => 259u32,
            crate::pac::rcc::vals::Plln::MUL260 => 260u32,
            crate::pac::rcc::vals::Plln::MUL261 => 261u32,
            crate::pac::rcc::vals::Plln::MUL262 => 262u32,
            crate::pac::rcc::vals::Plln::MUL263 => 263u32,
            crate::pac::rcc::vals::Plln::MUL264 => 264u32,
            crate::pac::rcc::vals::Plln::MUL265 => 265u32,
            crate::pac::rcc::vals::Plln::MUL266 => 266u32,
            crate::pac::rcc::vals::Plln::MUL267 => 267u32,
            crate::pac::rcc::vals::Plln::MUL268 => 268u32,
            crate::pac::rcc::vals::Plln::MUL269 => 269u32,
            crate::pac::rcc::vals::Plln::MUL270 => 270u32,
            crate::pac::rcc::vals::Plln::MUL271 => 271u32,
            crate::pac::rcc::vals::Plln::MUL272 => 272u32,
            crate::pac::rcc::vals::Plln::MUL273 => 273u32,
            crate::pac::rcc::vals::Plln::MUL274 => 274u32,
            crate::pac::rcc::vals::Plln::MUL275 => 275u32,
            crate::pac::rcc::vals::Plln::MUL276 => 276u32,
            crate::pac::rcc::vals::Plln::MUL277 => 277u32,
            crate::pac::rcc::vals::Plln::MUL278 => 278u32,
            crate::pac::rcc::vals::Plln::MUL279 => 279u32,
            crate::pac::rcc::vals::Plln::MUL280 => 280u32,
            crate::pac::rcc::vals::Plln::MUL281 => 281u32,
            crate::pac::rcc::vals::Plln::MUL282 => 282u32,
            crate::pac::rcc::vals::Plln::MUL283 => 283u32,
            crate::pac::rcc::vals::Plln::MUL284 => 284u32,
            crate::pac::rcc::vals::Plln::MUL285 => 285u32,
            crate::pac::rcc::vals::Plln::MUL286 => 286u32,
            crate::pac::rcc::vals::Plln::MUL287 => 287u32,
            crate::pac::rcc::vals::Plln::MUL288 => 288u32,
            crate::pac::rcc::vals::Plln::MUL289 => 289u32,
            crate::pac::rcc::vals::Plln::MUL290 => 290u32,
            crate::pac::rcc::vals::Plln::MUL291 => 291u32,
            crate::pac::rcc::vals::Plln::MUL292 => 292u32,
            crate::pac::rcc::vals::Plln::MUL293 => 293u32,
            crate::pac::rcc::vals::Plln::MUL294 => 294u32,
            crate::pac::rcc::vals::Plln::MUL295 => 295u32,
            crate::pac::rcc::vals::Plln::MUL296 => 296u32,
            crate::pac::rcc::vals::Plln::MUL297 => 297u32,
            crate::pac::rcc::vals::Plln::MUL298 => 298u32,
            crate::pac::rcc::vals::Plln::MUL299 => 299u32,
            crate::pac::rcc::vals::Plln::MUL300 => 300u32,
            crate::pac::rcc::vals::Plln::MUL301 => 301u32,
            crate::pac::rcc::vals::Plln::MUL302 => 302u32,
            crate::pac::rcc::vals::Plln::MUL303 => 303u32,
            crate::pac::rcc::vals::Plln::MUL304 => 304u32,
            crate::pac::rcc::vals::Plln::MUL305 => 305u32,
            crate::pac::rcc::vals::Plln::MUL306 => 306u32,
            crate::pac::rcc::vals::Plln::MUL307 => 307u32,
            crate::pac::rcc::vals::Plln::MUL308 => 308u32,
            crate::pac::rcc::vals::Plln::MUL309 => 309u32,
            crate::pac::rcc::vals::Plln::MUL310 => 310u32,
            crate::pac::rcc::vals::Plln::MUL311 => 311u32,
            crate::pac::rcc::vals::Plln::MUL312 => 312u32,
            crate::pac::rcc::vals::Plln::MUL313 => 313u32,
            crate::pac::rcc::vals::Plln::MUL314 => 314u32,
            crate::pac::rcc::vals::Plln::MUL315 => 315u32,
            crate::pac::rcc::vals::Plln::MUL316 => 316u32,
            crate::pac::rcc::vals::Plln::MUL317 => 317u32,
            crate::pac::rcc::vals::Plln::MUL318 => 318u32,
            crate::pac::rcc::vals::Plln::MUL319 => 319u32,
            crate::pac::rcc::vals::Plln::MUL320 => 320u32,
            crate::pac::rcc::vals::Plln::MUL321 => 321u32,
            crate::pac::rcc::vals::Plln::MUL322 => 322u32,
            crate::pac::rcc::vals::Plln::MUL323 => 323u32,
            crate::pac::rcc::vals::Plln::MUL324 => 324u32,
            crate::pac::rcc::vals::Plln::MUL325 => 325u32,
            crate::pac::rcc::vals::Plln::MUL326 => 326u32,
            crate::pac::rcc::vals::Plln::MUL327 => 327u32,
            crate::pac::rcc::vals::Plln::MUL328 => 328u32,
            crate::pac::rcc::vals::Plln::MUL329 => 329u32,
            crate::pac::rcc::vals::Plln::MUL330 => 330u32,
            crate::pac::rcc::vals::Plln::MUL331 => 331u32,
            crate::pac::rcc::vals::Plln::MUL332 => 332u32,
            crate::pac::rcc::vals::Plln::MUL333 => 333u32,
            crate::pac::rcc::vals::Plln::MUL334 => 334u32,
            crate::pac::rcc::vals::Plln::MUL335 => 335u32,
            crate::pac::rcc::vals::Plln::MUL336 => 336u32,
            crate::pac::rcc::vals::Plln::MUL337 => 337u32,
            crate::pac::rcc::vals::Plln::MUL338 => 338u32,
            crate::pac::rcc::vals::Plln::MUL339 => 339u32,
            crate::pac::rcc::vals::Plln::MUL340 => 340u32,
            crate::pac::rcc::vals::Plln::MUL341 => 341u32,
            crate::pac::rcc::vals::Plln::MUL342 => 342u32,
            crate::pac::rcc::vals::Plln::MUL343 => 343u32,
            crate::pac::rcc::vals::Plln::MUL344 => 344u32,
            crate::pac::rcc::vals::Plln::MUL345 => 345u32,
            crate::pac::rcc::vals::Plln::MUL346 => 346u32,
            crate::pac::rcc::vals::Plln::MUL347 => 347u32,
            crate::pac::rcc::vals::Plln::MUL348 => 348u32,
            crate::pac::rcc::vals::Plln::MUL349 => 349u32,
            crate::pac::rcc::vals::Plln::MUL350 => 350u32,
            crate::pac::rcc::vals::Plln::MUL351 => 351u32,
            crate::pac::rcc::vals::Plln::MUL352 => 352u32,
            crate::pac::rcc::vals::Plln::MUL353 => 353u32,
            crate::pac::rcc::vals::Plln::MUL354 => 354u32,
            crate::pac::rcc::vals::Plln::MUL355 => 355u32,
            crate::pac::rcc::vals::Plln::MUL356 => 356u32,
            crate::pac::rcc::vals::Plln::MUL357 => 357u32,
            crate::pac::rcc::vals::Plln::MUL358 => 358u32,
            crate::pac::rcc::vals::Plln::MUL359 => 359u32,
            crate::pac::rcc::vals::Plln::MUL360 => 360u32,
            crate::pac::rcc::vals::Plln::MUL361 => 361u32,
            crate::pac::rcc::vals::Plln::MUL362 => 362u32,
            crate::pac::rcc::vals::Plln::MUL363 => 363u32,
            crate::pac::rcc::vals::Plln::MUL364 => 364u32,
            crate::pac::rcc::vals::Plln::MUL365 => 365u32,
            crate::pac::rcc::vals::Plln::MUL366 => 366u32,
            crate::pac::rcc::vals::Plln::MUL367 => 367u32,
            crate::pac::rcc::vals::Plln::MUL368 => 368u32,
            crate::pac::rcc::vals::Plln::MUL369 => 369u32,
            crate::pac::rcc::vals::Plln::MUL370 => 370u32,
            crate::pac::rcc::vals::Plln::MUL371 => 371u32,
            crate::pac::rcc::vals::Plln::MUL372 => 372u32,
            crate::pac::rcc::vals::Plln::MUL373 => 373u32,
            crate::pac::rcc::vals::Plln::MUL374 => 374u32,
            crate::pac::rcc::vals::Plln::MUL375 => 375u32,
            crate::pac::rcc::vals::Plln::MUL376 => 376u32,
            crate::pac::rcc::vals::Plln::MUL377 => 377u32,
            crate::pac::rcc::vals::Plln::MUL378 => 378u32,
            crate::pac::rcc::vals::Plln::MUL379 => 379u32,
            crate::pac::rcc::vals::Plln::MUL380 => 380u32,
            crate::pac::rcc::vals::Plln::MUL381 => 381u32,
            crate::pac::rcc::vals::Plln::MUL382 => 382u32,
            crate::pac::rcc::vals::Plln::MUL383 => 383u32,
            crate::pac::rcc::vals::Plln::MUL384 => 384u32,
            crate::pac::rcc::vals::Plln::MUL385 => 385u32,
            crate::pac::rcc::vals::Plln::MUL386 => 386u32,
            crate::pac::rcc::vals::Plln::MUL387 => 387u32,
            crate::pac::rcc::vals::Plln::MUL388 => 388u32,
            crate::pac::rcc::vals::Plln::MUL389 => 389u32,
            crate::pac::rcc::vals::Plln::MUL390 => 390u32,
            crate::pac::rcc::vals::Plln::MUL391 => 391u32,
            crate::pac::rcc::vals::Plln::MUL392 => 392u32,
            crate::pac::rcc::vals::Plln::MUL393 => 393u32,
            crate::pac::rcc::vals::Plln::MUL394 => 394u32,
            crate::pac::rcc::vals::Plln::MUL395 => 395u32,
            crate::pac::rcc::vals::Plln::MUL396 => 396u32,
            crate::pac::rcc::vals::Plln::MUL397 => 397u32,
            crate::pac::rcc::vals::Plln::MUL398 => 398u32,
            crate::pac::rcc::vals::Plln::MUL399 => 399u32,
            crate::pac::rcc::vals::Plln::MUL400 => 400u32,
            crate::pac::rcc::vals::Plln::MUL401 => 401u32,
            crate::pac::rcc::vals::Plln::MUL402 => 402u32,
            crate::pac::rcc::vals::Plln::MUL403 => 403u32,
            crate::pac::rcc::vals::Plln::MUL404 => 404u32,
            crate::pac::rcc::vals::Plln::MUL405 => 405u32,
            crate::pac::rcc::vals::Plln::MUL406 => 406u32,
            crate::pac::rcc::vals::Plln::MUL407 => 407u32,
            crate::pac::rcc::vals::Plln::MUL408 => 408u32,
            crate::pac::rcc::vals::Plln::MUL409 => 409u32,
            crate::pac::rcc::vals::Plln::MUL410 => 410u32,
            crate::pac::rcc::vals::Plln::MUL411 => 411u32,
            crate::pac::rcc::vals::Plln::MUL412 => 412u32,
            crate::pac::rcc::vals::Plln::MUL413 => 413u32,
            crate::pac::rcc::vals::Plln::MUL414 => 414u32,
            crate::pac::rcc::vals::Plln::MUL415 => 415u32,
            crate::pac::rcc::vals::Plln::MUL416 => 416u32,
            crate::pac::rcc::vals::Plln::MUL417 => 417u32,
            crate::pac::rcc::vals::Plln::MUL418 => 418u32,
            crate::pac::rcc::vals::Plln::MUL419 => 419u32,
            crate::pac::rcc::vals::Plln::MUL420 => 420u32,
            crate::pac::rcc::vals::Plln::MUL421 => 421u32,
            crate::pac::rcc::vals::Plln::MUL422 => 422u32,
            crate::pac::rcc::vals::Plln::MUL423 => 423u32,
            crate::pac::rcc::vals::Plln::MUL424 => 424u32,
            crate::pac::rcc::vals::Plln::MUL425 => 425u32,
            crate::pac::rcc::vals::Plln::MUL426 => 426u32,
            crate::pac::rcc::vals::Plln::MUL427 => 427u32,
            crate::pac::rcc::vals::Plln::MUL428 => 428u32,
            crate::pac::rcc::vals::Plln::MUL429 => 429u32,
            crate::pac::rcc::vals::Plln::MUL430 => 430u32,
            crate::pac::rcc::vals::Plln::MUL431 => 431u32,
            crate::pac::rcc::vals::Plln::MUL432 => 432u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plln::MUL50 => 1u32,
            crate::pac::rcc::vals::Plln::MUL51 => 1u32,
            crate::pac::rcc::vals::Plln::MUL52 => 1u32,
            crate::pac::rcc::vals::Plln::MUL53 => 1u32,
            crate::pac::rcc::vals::Plln::MUL54 => 1u32,
            crate::pac::rcc::vals::Plln::MUL55 => 1u32,
            crate::pac::rcc::vals::Plln::MUL56 => 1u32,
            crate::pac::rcc::vals::Plln::MUL57 => 1u32,
            crate::pac::rcc::vals::Plln::MUL58 => 1u32,
            crate::pac::rcc::vals::Plln::MUL59 => 1u32,
            crate::pac::rcc::vals::Plln::MUL60 => 1u32,
            crate::pac::rcc::vals::Plln::MUL61 => 1u32,
            crate::pac::rcc::vals::Plln::MUL62 => 1u32,
            crate::pac::rcc::vals::Plln::MUL63 => 1u32,
            crate::pac::rcc::vals::Plln::MUL64 => 1u32,
            crate::pac::rcc::vals::Plln::MUL65 => 1u32,
            crate::pac::rcc::vals::Plln::MUL66 => 1u32,
            crate::pac::rcc::vals::Plln::MUL67 => 1u32,
            crate::pac::rcc::vals::Plln::MUL68 => 1u32,
            crate::pac::rcc::vals::Plln::MUL69 => 1u32,
            crate::pac::rcc::vals::Plln::MUL70 => 1u32,
            crate::pac::rcc::vals::Plln::MUL71 => 1u32,
            crate::pac::rcc::vals::Plln::MUL72 => 1u32,
            crate::pac::rcc::vals::Plln::MUL73 => 1u32,
            crate::pac::rcc::vals::Plln::MUL74 => 1u32,
            crate::pac::rcc::vals::Plln::MUL75 => 1u32,
            crate::pac::rcc::vals::Plln::MUL76 => 1u32,
            crate::pac::rcc::vals::Plln::MUL77 => 1u32,
            crate::pac::rcc::vals::Plln::MUL78 => 1u32,
            crate::pac::rcc::vals::Plln::MUL79 => 1u32,
            crate::pac::rcc::vals::Plln::MUL80 => 1u32,
            crate::pac::rcc::vals::Plln::MUL81 => 1u32,
            crate::pac::rcc::vals::Plln::MUL82 => 1u32,
            crate::pac::rcc::vals::Plln::MUL83 => 1u32,
            crate::pac::rcc::vals::Plln::MUL84 => 1u32,
            crate::pac::rcc::vals::Plln::MUL85 => 1u32,
            crate::pac::rcc::vals::Plln::MUL86 => 1u32,
            crate::pac::rcc::vals::Plln::MUL87 => 1u32,
            crate::pac::rcc::vals::Plln::MUL88 => 1u32,
            crate::pac::rcc::vals::Plln::MUL89 => 1u32,
            crate::pac::rcc::vals::Plln::MUL90 => 1u32,
            crate::pac::rcc::vals::Plln::MUL91 => 1u32,
            crate::pac::rcc::vals::Plln::MUL92 => 1u32,
            crate::pac::rcc::vals::Plln::MUL93 => 1u32,
            crate::pac::rcc::vals::Plln::MUL94 => 1u32,
            crate::pac::rcc::vals::Plln::MUL95 => 1u32,
            crate::pac::rcc::vals::Plln::MUL96 => 1u32,
            crate::pac::rcc::vals::Plln::MUL97 => 1u32,
            crate::pac::rcc::vals::Plln::MUL98 => 1u32,
            crate::pac::rcc::vals::Plln::MUL99 => 1u32,
            crate::pac::rcc::vals::Plln::MUL100 => 1u32,
            crate::pac::rcc::vals::Plln::MUL101 => 1u32,
            crate::pac::rcc::vals::Plln::MUL102 => 1u32,
            crate::pac::rcc::vals::Plln::MUL103 => 1u32,
            crate::pac::rcc::vals::Plln::MUL104 => 1u32,
            crate::pac::rcc::vals::Plln::MUL105 => 1u32,
            crate::pac::rcc::vals::Plln::MUL106 => 1u32,
            crate::pac::rcc::vals::Plln::MUL107 => 1u32,
            crate::pac::rcc::vals::Plln::MUL108 => 1u32,
            crate::pac::rcc::vals::Plln::MUL109 => 1u32,
            crate::pac::rcc::vals::Plln::MUL110 => 1u32,
            crate::pac::rcc::vals::Plln::MUL111 => 1u32,
            crate::pac::rcc::vals::Plln::MUL112 => 1u32,
            crate::pac::rcc::vals::Plln::MUL113 => 1u32,
            crate::pac::rcc::vals::Plln::MUL114 => 1u32,
            crate::pac::rcc::vals::Plln::MUL115 => 1u32,
            crate::pac::rcc::vals::Plln::MUL116 => 1u32,
            crate::pac::rcc::vals::Plln::MUL117 => 1u32,
            crate::pac::rcc::vals::Plln::MUL118 => 1u32,
            crate::pac::rcc::vals::Plln::MUL119 => 1u32,
            crate::pac::rcc::vals::Plln::MUL120 => 1u32,
            crate::pac::rcc::vals::Plln::MUL121 => 1u32,
            crate::pac::rcc::vals::Plln::MUL122 => 1u32,
            crate::pac::rcc::vals::Plln::MUL123 => 1u32,
            crate::pac::rcc::vals::Plln::MUL124 => 1u32,
            crate::pac::rcc::vals::Plln::MUL125 => 1u32,
            crate::pac::rcc::vals::Plln::MUL126 => 1u32,
            crate::pac::rcc::vals::Plln::MUL127 => 1u32,
            crate::pac::rcc::vals::Plln::MUL128 => 1u32,
            crate::pac::rcc::vals::Plln::MUL129 => 1u32,
            crate::pac::rcc::vals::Plln::MUL130 => 1u32,
            crate::pac::rcc::vals::Plln::MUL131 => 1u32,
            crate::pac::rcc::vals::Plln::MUL132 => 1u32,
            crate::pac::rcc::vals::Plln::MUL133 => 1u32,
            crate::pac::rcc::vals::Plln::MUL134 => 1u32,
            crate::pac::rcc::vals::Plln::MUL135 => 1u32,
            crate::pac::rcc::vals::Plln::MUL136 => 1u32,
            crate::pac::rcc::vals::Plln::MUL137 => 1u32,
            crate::pac::rcc::vals::Plln::MUL138 => 1u32,
            crate::pac::rcc::vals::Plln::MUL139 => 1u32,
            crate::pac::rcc::vals::Plln::MUL140 => 1u32,
            crate::pac::rcc::vals::Plln::MUL141 => 1u32,
            crate::pac::rcc::vals::Plln::MUL142 => 1u32,
            crate::pac::rcc::vals::Plln::MUL143 => 1u32,
            crate::pac::rcc::vals::Plln::MUL144 => 1u32,
            crate::pac::rcc::vals::Plln::MUL145 => 1u32,
            crate::pac::rcc::vals::Plln::MUL146 => 1u32,
            crate::pac::rcc::vals::Plln::MUL147 => 1u32,
            crate::pac::rcc::vals::Plln::MUL148 => 1u32,
            crate::pac::rcc::vals::Plln::MUL149 => 1u32,
            crate::pac::rcc::vals::Plln::MUL150 => 1u32,
            crate::pac::rcc::vals::Plln::MUL151 => 1u32,
            crate::pac::rcc::vals::Plln::MUL152 => 1u32,
            crate::pac::rcc::vals::Plln::MUL153 => 1u32,
            crate::pac::rcc::vals::Plln::MUL154 => 1u32,
            crate::pac::rcc::vals::Plln::MUL155 => 1u32,
            crate::pac::rcc::vals::Plln::MUL156 => 1u32,
            crate::pac::rcc::vals::Plln::MUL157 => 1u32,
            crate::pac::rcc::vals::Plln::MUL158 => 1u32,
            crate::pac::rcc::vals::Plln::MUL159 => 1u32,
            crate::pac::rcc::vals::Plln::MUL160 => 1u32,
            crate::pac::rcc::vals::Plln::MUL161 => 1u32,
            crate::pac::rcc::vals::Plln::MUL162 => 1u32,
            crate::pac::rcc::vals::Plln::MUL163 => 1u32,
            crate::pac::rcc::vals::Plln::MUL164 => 1u32,
            crate::pac::rcc::vals::Plln::MUL165 => 1u32,
            crate::pac::rcc::vals::Plln::MUL166 => 1u32,
            crate::pac::rcc::vals::Plln::MUL167 => 1u32,
            crate::pac::rcc::vals::Plln::MUL168 => 1u32,
            crate::pac::rcc::vals::Plln::MUL169 => 1u32,
            crate::pac::rcc::vals::Plln::MUL170 => 1u32,
            crate::pac::rcc::vals::Plln::MUL171 => 1u32,
            crate::pac::rcc::vals::Plln::MUL172 => 1u32,
            crate::pac::rcc::vals::Plln::MUL173 => 1u32,
            crate::pac::rcc::vals::Plln::MUL174 => 1u32,
            crate::pac::rcc::vals::Plln::MUL175 => 1u32,
            crate::pac::rcc::vals::Plln::MUL176 => 1u32,
            crate::pac::rcc::vals::Plln::MUL177 => 1u32,
            crate::pac::rcc::vals::Plln::MUL178 => 1u32,
            crate::pac::rcc::vals::Plln::MUL179 => 1u32,
            crate::pac::rcc::vals::Plln::MUL180 => 1u32,
            crate::pac::rcc::vals::Plln::MUL181 => 1u32,
            crate::pac::rcc::vals::Plln::MUL182 => 1u32,
            crate::pac::rcc::vals::Plln::MUL183 => 1u32,
            crate::pac::rcc::vals::Plln::MUL184 => 1u32,
            crate::pac::rcc::vals::Plln::MUL185 => 1u32,
            crate::pac::rcc::vals::Plln::MUL186 => 1u32,
            crate::pac::rcc::vals::Plln::MUL187 => 1u32,
            crate::pac::rcc::vals::Plln::MUL188 => 1u32,
            crate::pac::rcc::vals::Plln::MUL189 => 1u32,
            crate::pac::rcc::vals::Plln::MUL190 => 1u32,
            crate::pac::rcc::vals::Plln::MUL191 => 1u32,
            crate::pac::rcc::vals::Plln::MUL192 => 1u32,
            crate::pac::rcc::vals::Plln::MUL193 => 1u32,
            crate::pac::rcc::vals::Plln::MUL194 => 1u32,
            crate::pac::rcc::vals::Plln::MUL195 => 1u32,
            crate::pac::rcc::vals::Plln::MUL196 => 1u32,
            crate::pac::rcc::vals::Plln::MUL197 => 1u32,
            crate::pac::rcc::vals::Plln::MUL198 => 1u32,
            crate::pac::rcc::vals::Plln::MUL199 => 1u32,
            crate::pac::rcc::vals::Plln::MUL200 => 1u32,
            crate::pac::rcc::vals::Plln::MUL201 => 1u32,
            crate::pac::rcc::vals::Plln::MUL202 => 1u32,
            crate::pac::rcc::vals::Plln::MUL203 => 1u32,
            crate::pac::rcc::vals::Plln::MUL204 => 1u32,
            crate::pac::rcc::vals::Plln::MUL205 => 1u32,
            crate::pac::rcc::vals::Plln::MUL206 => 1u32,
            crate::pac::rcc::vals::Plln::MUL207 => 1u32,
            crate::pac::rcc::vals::Plln::MUL208 => 1u32,
            crate::pac::rcc::vals::Plln::MUL209 => 1u32,
            crate::pac::rcc::vals::Plln::MUL210 => 1u32,
            crate::pac::rcc::vals::Plln::MUL211 => 1u32,
            crate::pac::rcc::vals::Plln::MUL212 => 1u32,
            crate::pac::rcc::vals::Plln::MUL213 => 1u32,
            crate::pac::rcc::vals::Plln::MUL214 => 1u32,
            crate::pac::rcc::vals::Plln::MUL215 => 1u32,
            crate::pac::rcc::vals::Plln::MUL216 => 1u32,
            crate::pac::rcc::vals::Plln::MUL217 => 1u32,
            crate::pac::rcc::vals::Plln::MUL218 => 1u32,
            crate::pac::rcc::vals::Plln::MUL219 => 1u32,
            crate::pac::rcc::vals::Plln::MUL220 => 1u32,
            crate::pac::rcc::vals::Plln::MUL221 => 1u32,
            crate::pac::rcc::vals::Plln::MUL222 => 1u32,
            crate::pac::rcc::vals::Plln::MUL223 => 1u32,
            crate::pac::rcc::vals::Plln::MUL224 => 1u32,
            crate::pac::rcc::vals::Plln::MUL225 => 1u32,
            crate::pac::rcc::vals::Plln::MUL226 => 1u32,
            crate::pac::rcc::vals::Plln::MUL227 => 1u32,
            crate::pac::rcc::vals::Plln::MUL228 => 1u32,
            crate::pac::rcc::vals::Plln::MUL229 => 1u32,
            crate::pac::rcc::vals::Plln::MUL230 => 1u32,
            crate::pac::rcc::vals::Plln::MUL231 => 1u32,
            crate::pac::rcc::vals::Plln::MUL232 => 1u32,
            crate::pac::rcc::vals::Plln::MUL233 => 1u32,
            crate::pac::rcc::vals::Plln::MUL234 => 1u32,
            crate::pac::rcc::vals::Plln::MUL235 => 1u32,
            crate::pac::rcc::vals::Plln::MUL236 => 1u32,
            crate::pac::rcc::vals::Plln::MUL237 => 1u32,
            crate::pac::rcc::vals::Plln::MUL238 => 1u32,
            crate::pac::rcc::vals::Plln::MUL239 => 1u32,
            crate::pac::rcc::vals::Plln::MUL240 => 1u32,
            crate::pac::rcc::vals::Plln::MUL241 => 1u32,
            crate::pac::rcc::vals::Plln::MUL242 => 1u32,
            crate::pac::rcc::vals::Plln::MUL243 => 1u32,
            crate::pac::rcc::vals::Plln::MUL244 => 1u32,
            crate::pac::rcc::vals::Plln::MUL245 => 1u32,
            crate::pac::rcc::vals::Plln::MUL246 => 1u32,
            crate::pac::rcc::vals::Plln::MUL247 => 1u32,
            crate::pac::rcc::vals::Plln::MUL248 => 1u32,
            crate::pac::rcc::vals::Plln::MUL249 => 1u32,
            crate::pac::rcc::vals::Plln::MUL250 => 1u32,
            crate::pac::rcc::vals::Plln::MUL251 => 1u32,
            crate::pac::rcc::vals::Plln::MUL252 => 1u32,
            crate::pac::rcc::vals::Plln::MUL253 => 1u32,
            crate::pac::rcc::vals::Plln::MUL254 => 1u32,
            crate::pac::rcc::vals::Plln::MUL255 => 1u32,
            crate::pac::rcc::vals::Plln::MUL256 => 1u32,
            crate::pac::rcc::vals::Plln::MUL257 => 1u32,
            crate::pac::rcc::vals::Plln::MUL258 => 1u32,
            crate::pac::rcc::vals::Plln::MUL259 => 1u32,
            crate::pac::rcc::vals::Plln::MUL260 => 1u32,
            crate::pac::rcc::vals::Plln::MUL261 => 1u32,
            crate::pac::rcc::vals::Plln::MUL262 => 1u32,
            crate::pac::rcc::vals::Plln::MUL263 => 1u32,
            crate::pac::rcc::vals::Plln::MUL264 => 1u32,
            crate::pac::rcc::vals::Plln::MUL265 => 1u32,
            crate::pac::rcc::vals::Plln::MUL266 => 1u32,
            crate::pac::rcc::vals::Plln::MUL267 => 1u32,
            crate::pac::rcc::vals::Plln::MUL268 => 1u32,
            crate::pac::rcc::vals::Plln::MUL269 => 1u32,
            crate::pac::rcc::vals::Plln::MUL270 => 1u32,
            crate::pac::rcc::vals::Plln::MUL271 => 1u32,
            crate::pac::rcc::vals::Plln::MUL272 => 1u32,
            crate::pac::rcc::vals::Plln::MUL273 => 1u32,
            crate::pac::rcc::vals::Plln::MUL274 => 1u32,
            crate::pac::rcc::vals::Plln::MUL275 => 1u32,
            crate::pac::rcc::vals::Plln::MUL276 => 1u32,
            crate::pac::rcc::vals::Plln::MUL277 => 1u32,
            crate::pac::rcc::vals::Plln::MUL278 => 1u32,
            crate::pac::rcc::vals::Plln::MUL279 => 1u32,
            crate::pac::rcc::vals::Plln::MUL280 => 1u32,
            crate::pac::rcc::vals::Plln::MUL281 => 1u32,
            crate::pac::rcc::vals::Plln::MUL282 => 1u32,
            crate::pac::rcc::vals::Plln::MUL283 => 1u32,
            crate::pac::rcc::vals::Plln::MUL284 => 1u32,
            crate::pac::rcc::vals::Plln::MUL285 => 1u32,
            crate::pac::rcc::vals::Plln::MUL286 => 1u32,
            crate::pac::rcc::vals::Plln::MUL287 => 1u32,
            crate::pac::rcc::vals::Plln::MUL288 => 1u32,
            crate::pac::rcc::vals::Plln::MUL289 => 1u32,
            crate::pac::rcc::vals::Plln::MUL290 => 1u32,
            crate::pac::rcc::vals::Plln::MUL291 => 1u32,
            crate::pac::rcc::vals::Plln::MUL292 => 1u32,
            crate::pac::rcc::vals::Plln::MUL293 => 1u32,
            crate::pac::rcc::vals::Plln::MUL294 => 1u32,
            crate::pac::rcc::vals::Plln::MUL295 => 1u32,
            crate::pac::rcc::vals::Plln::MUL296 => 1u32,
            crate::pac::rcc::vals::Plln::MUL297 => 1u32,
            crate::pac::rcc::vals::Plln::MUL298 => 1u32,
            crate::pac::rcc::vals::Plln::MUL299 => 1u32,
            crate::pac::rcc::vals::Plln::MUL300 => 1u32,
            crate::pac::rcc::vals::Plln::MUL301 => 1u32,
            crate::pac::rcc::vals::Plln::MUL302 => 1u32,
            crate::pac::rcc::vals::Plln::MUL303 => 1u32,
            crate::pac::rcc::vals::Plln::MUL304 => 1u32,
            crate::pac::rcc::vals::Plln::MUL305 => 1u32,
            crate::pac::rcc::vals::Plln::MUL306 => 1u32,
            crate::pac::rcc::vals::Plln::MUL307 => 1u32,
            crate::pac::rcc::vals::Plln::MUL308 => 1u32,
            crate::pac::rcc::vals::Plln::MUL309 => 1u32,
            crate::pac::rcc::vals::Plln::MUL310 => 1u32,
            crate::pac::rcc::vals::Plln::MUL311 => 1u32,
            crate::pac::rcc::vals::Plln::MUL312 => 1u32,
            crate::pac::rcc::vals::Plln::MUL313 => 1u32,
            crate::pac::rcc::vals::Plln::MUL314 => 1u32,
            crate::pac::rcc::vals::Plln::MUL315 => 1u32,
            crate::pac::rcc::vals::Plln::MUL316 => 1u32,
            crate::pac::rcc::vals::Plln::MUL317 => 1u32,
            crate::pac::rcc::vals::Plln::MUL318 => 1u32,
            crate::pac::rcc::vals::Plln::MUL319 => 1u32,
            crate::pac::rcc::vals::Plln::MUL320 => 1u32,
            crate::pac::rcc::vals::Plln::MUL321 => 1u32,
            crate::pac::rcc::vals::Plln::MUL322 => 1u32,
            crate::pac::rcc::vals::Plln::MUL323 => 1u32,
            crate::pac::rcc::vals::Plln::MUL324 => 1u32,
            crate::pac::rcc::vals::Plln::MUL325 => 1u32,
            crate::pac::rcc::vals::Plln::MUL326 => 1u32,
            crate::pac::rcc::vals::Plln::MUL327 => 1u32,
            crate::pac::rcc::vals::Plln::MUL328 => 1u32,
            crate::pac::rcc::vals::Plln::MUL329 => 1u32,
            crate::pac::rcc::vals::Plln::MUL330 => 1u32,
            crate::pac::rcc::vals::Plln::MUL331 => 1u32,
            crate::pac::rcc::vals::Plln::MUL332 => 1u32,
            crate::pac::rcc::vals::Plln::MUL333 => 1u32,
            crate::pac::rcc::vals::Plln::MUL334 => 1u32,
            crate::pac::rcc::vals::Plln::MUL335 => 1u32,
            crate::pac::rcc::vals::Plln::MUL336 => 1u32,
            crate::pac::rcc::vals::Plln::MUL337 => 1u32,
            crate::pac::rcc::vals::Plln::MUL338 => 1u32,
            crate::pac::rcc::vals::Plln::MUL339 => 1u32,
            crate::pac::rcc::vals::Plln::MUL340 => 1u32,
            crate::pac::rcc::vals::Plln::MUL341 => 1u32,
            crate::pac::rcc::vals::Plln::MUL342 => 1u32,
            crate::pac::rcc::vals::Plln::MUL343 => 1u32,
            crate::pac::rcc::vals::Plln::MUL344 => 1u32,
            crate::pac::rcc::vals::Plln::MUL345 => 1u32,
            crate::pac::rcc::vals::Plln::MUL346 => 1u32,
            crate::pac::rcc::vals::Plln::MUL347 => 1u32,
            crate::pac::rcc::vals::Plln::MUL348 => 1u32,
            crate::pac::rcc::vals::Plln::MUL349 => 1u32,
            crate::pac::rcc::vals::Plln::MUL350 => 1u32,
            crate::pac::rcc::vals::Plln::MUL351 => 1u32,
            crate::pac::rcc::vals::Plln::MUL352 => 1u32,
            crate::pac::rcc::vals::Plln::MUL353 => 1u32,
            crate::pac::rcc::vals::Plln::MUL354 => 1u32,
            crate::pac::rcc::vals::Plln::MUL355 => 1u32,
            crate::pac::rcc::vals::Plln::MUL356 => 1u32,
            crate::pac::rcc::vals::Plln::MUL357 => 1u32,
            crate::pac::rcc::vals::Plln::MUL358 => 1u32,
            crate::pac::rcc::vals::Plln::MUL359 => 1u32,
            crate::pac::rcc::vals::Plln::MUL360 => 1u32,
            crate::pac::rcc::vals::Plln::MUL361 => 1u32,
            crate::pac::rcc::vals::Plln::MUL362 => 1u32,
            crate::pac::rcc::vals::Plln::MUL363 => 1u32,
            crate::pac::rcc::vals::Plln::MUL364 => 1u32,
            crate::pac::rcc::vals::Plln::MUL365 => 1u32,
            crate::pac::rcc::vals::Plln::MUL366 => 1u32,
            crate::pac::rcc::vals::Plln::MUL367 => 1u32,
            crate::pac::rcc::vals::Plln::MUL368 => 1u32,
            crate::pac::rcc::vals::Plln::MUL369 => 1u32,
            crate::pac::rcc::vals::Plln::MUL370 => 1u32,
            crate::pac::rcc::vals::Plln::MUL371 => 1u32,
            crate::pac::rcc::vals::Plln::MUL372 => 1u32,
            crate::pac::rcc::vals::Plln::MUL373 => 1u32,
            crate::pac::rcc::vals::Plln::MUL374 => 1u32,
            crate::pac::rcc::vals::Plln::MUL375 => 1u32,
            crate::pac::rcc::vals::Plln::MUL376 => 1u32,
            crate::pac::rcc::vals::Plln::MUL377 => 1u32,
            crate::pac::rcc::vals::Plln::MUL378 => 1u32,
            crate::pac::rcc::vals::Plln::MUL379 => 1u32,
            crate::pac::rcc::vals::Plln::MUL380 => 1u32,
            crate::pac::rcc::vals::Plln::MUL381 => 1u32,
            crate::pac::rcc::vals::Plln::MUL382 => 1u32,
            crate::pac::rcc::vals::Plln::MUL383 => 1u32,
            crate::pac::rcc::vals::Plln::MUL384 => 1u32,
            crate::pac::rcc::vals::Plln::MUL385 => 1u32,
            crate::pac::rcc::vals::Plln::MUL386 => 1u32,
            crate::pac::rcc::vals::Plln::MUL387 => 1u32,
            crate::pac::rcc::vals::Plln::MUL388 => 1u32,
            crate::pac::rcc::vals::Plln::MUL389 => 1u32,
            crate::pac::rcc::vals::Plln::MUL390 => 1u32,
            crate::pac::rcc::vals::Plln::MUL391 => 1u32,
            crate::pac::rcc::vals::Plln::MUL392 => 1u32,
            crate::pac::rcc::vals::Plln::MUL393 => 1u32,
            crate::pac::rcc::vals::Plln::MUL394 => 1u32,
            crate::pac::rcc::vals::Plln::MUL395 => 1u32,
            crate::pac::rcc::vals::Plln::MUL396 => 1u32,
            crate::pac::rcc::vals::Plln::MUL397 => 1u32,
            crate::pac::rcc::vals::Plln::MUL398 => 1u32,
            crate::pac::rcc::vals::Plln::MUL399 => 1u32,
            crate::pac::rcc::vals::Plln::MUL400 => 1u32,
            crate::pac::rcc::vals::Plln::MUL401 => 1u32,
            crate::pac::rcc::vals::Plln::MUL402 => 1u32,
            crate::pac::rcc::vals::Plln::MUL403 => 1u32,
            crate::pac::rcc::vals::Plln::MUL404 => 1u32,
            crate::pac::rcc::vals::Plln::MUL405 => 1u32,
            crate::pac::rcc::vals::Plln::MUL406 => 1u32,
            crate::pac::rcc::vals::Plln::MUL407 => 1u32,
            crate::pac::rcc::vals::Plln::MUL408 => 1u32,
            crate::pac::rcc::vals::Plln::MUL409 => 1u32,
            crate::pac::rcc::vals::Plln::MUL410 => 1u32,
            crate::pac::rcc::vals::Plln::MUL411 => 1u32,
            crate::pac::rcc::vals::Plln::MUL412 => 1u32,
            crate::pac::rcc::vals::Plln::MUL413 => 1u32,
            crate::pac::rcc::vals::Plln::MUL414 => 1u32,
            crate::pac::rcc::vals::Plln::MUL415 => 1u32,
            crate::pac::rcc::vals::Plln::MUL416 => 1u32,
            crate::pac::rcc::vals::Plln::MUL417 => 1u32,
            crate::pac::rcc::vals::Plln::MUL418 => 1u32,
            crate::pac::rcc::vals::Plln::MUL419 => 1u32,
            crate::pac::rcc::vals::Plln::MUL420 => 1u32,
            crate::pac::rcc::vals::Plln::MUL421 => 1u32,
            crate::pac::rcc::vals::Plln::MUL422 => 1u32,
            crate::pac::rcc::vals::Plln::MUL423 => 1u32,
            crate::pac::rcc::vals::Plln::MUL424 => 1u32,
            crate::pac::rcc::vals::Plln::MUL425 => 1u32,
            crate::pac::rcc::vals::Plln::MUL426 => 1u32,
            crate::pac::rcc::vals::Plln::MUL427 => 1u32,
            crate::pac::rcc::vals::Plln::MUL428 => 1u32,
            crate::pac::rcc::vals::Plln::MUL429 => 1u32,
            crate::pac::rcc::vals::Plln::MUL430 => 1u32,
            crate::pac::rcc::vals::Plln::MUL431 => 1u32,
            crate::pac::rcc::vals::Plln::MUL432 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllp {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllp::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllp::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllp::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllp::DIV8 => 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllp::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllp::DIV8 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllq {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllq::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllq::DIV3 => 3u32,
            crate::pac::rcc::vals::Pllq::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllq::DIV5 => 5u32,
            crate::pac::rcc::vals::Pllq::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllq::DIV7 => 7u32,
            crate::pac::rcc::vals::Pllq::DIV8 => 8u32,
            crate::pac::rcc::vals::Pllq::DIV9 => 9u32,
            crate::pac::rcc::vals::Pllq::DIV10 => 10u32,
            crate::pac::rcc::vals::Pllq::DIV11 => 11u32,
            crate::pac::rcc::vals::Pllq::DIV12 => 12u32,
            crate::pac::rcc::vals::Pllq::DIV13 => 13u32,
            crate::pac::rcc::vals::Pllq::DIV14 => 14u32,
            crate::pac::rcc::vals::Pllq::DIV15 => 15u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllq::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV3 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV5 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV7 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV8 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV9 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV10 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV11 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV12 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV13 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV14 => 1u32,
            crate::pac::rcc::vals::Pllq::DIV15 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllr {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllr::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllr::DIV3 => 3u32,
            crate::pac::rcc::vals::Pllr::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllr::DIV5 => 5u32,
            crate::pac::rcc::vals::Pllr::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllr::DIV7 => 7u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllr::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV3 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV5 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllr::DIV7 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Ppre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Ppre::DIV1 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Ppre::DIV1 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::adccommon::vals::Adcpre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::adccommon::vals::Adcpre::DIV2 => 2u32,
            crate::pac::adccommon::vals::Adcpre::DIV4 => 4u32,
            crate::pac::adccommon::vals::Adcpre::DIV6 => 6u32,
            crate::pac::adccommon::vals::Adcpre::DIV8 => 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::adccommon::vals::Adcpre::DIV2 => 1u32,
            crate::pac::adccommon::vals::Adcpre::DIV4 => 1u32,
            crate::pac::adccommon::vals::Adcpre::DIV6 => 1u32,
            crate::pac::adccommon::vals::Adcpre::DIV8 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod ADC123_COMMON {}
    pub mod ADC2 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod ADC3 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod CAN1 {
        pub type RX0 = crate::interrupt::typelevel::CAN1_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN1_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN1_SCE;
        pub type TX = crate::interrupt::typelevel::CAN1_TX;
    }
    pub mod CAN2 {
        pub type RX0 = crate::interrupt::typelevel::CAN2_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN2_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN2_SCE;
        pub type TX = crate::interrupt::typelevel::CAN2_TX;
    }
    pub mod CEC {
        pub type GLOBAL = crate::interrupt::typelevel::CEC;
    }
    pub mod CRC {}
    pub mod DAC1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod DBGMCU {}
    pub mod DCMI {
        pub type GLOBAL = crate::interrupt::typelevel::DCMI;
    }
    pub mod DMA1 {
        pub type CH0 = crate::interrupt::typelevel::DMA1_STREAM0;
        pub type CH1 = crate::interrupt::typelevel::DMA1_STREAM1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_STREAM2;
        pub type CH3 = crate::interrupt::typelevel::DMA1_STREAM3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_STREAM4;
        pub type CH5 = crate::interrupt::typelevel::DMA1_STREAM5;
        pub type CH6 = crate::interrupt::typelevel::DMA1_STREAM6;
        pub type CH7 = crate::interrupt::typelevel::DMA1_STREAM7;
    }
    pub mod DMA2 {
        pub type CH0 = crate::interrupt::typelevel::DMA2_STREAM0;
        pub type CH1 = crate::interrupt::typelevel::DMA2_STREAM1;
        pub type CH2 = crate::interrupt::typelevel::DMA2_STREAM2;
        pub type CH3 = crate::interrupt::typelevel::DMA2_STREAM3;
        pub type CH4 = crate::interrupt::typelevel::DMA2_STREAM4;
        pub type CH5 = crate::interrupt::typelevel::DMA2_STREAM5;
        pub type CH6 = crate::interrupt::typelevel::DMA2_STREAM6;
        pub type CH7 = crate::interrupt::typelevel::DMA2_STREAM7;
    }
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI9_5;
    }
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod FMPI2C1 {
        pub type ER = crate::interrupt::typelevel::FMPI2C1_ER;
        pub type EV = crate::interrupt::typelevel::FMPI2C1_EV;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOE {}
    pub mod GPIOF {}
    pub mod GPIOG {}
    pub mod GPIOH {}
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1_ER;
        pub type EV = crate::interrupt::typelevel::I2C1_EV;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2_ER;
        pub type EV = crate::interrupt::typelevel::I2C2_EV;
    }
    pub mod I2C3 {
        pub type ER = crate::interrupt::typelevel::I2C3_ER;
        pub type EV = crate::interrupt::typelevel::I2C3_EV;
    }
    pub mod I2S2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod I2S3 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI3;
    }
    pub mod IWDG {}
    pub mod PWR {}
    pub mod QUADSPI {
        pub type GLOBAL = crate::interrupt::typelevel::QUADSPI;
    }
    pub mod RCC {
        pub type GLOBAL = crate::interrupt::typelevel::RCC;
    }
    pub mod RTC {
        pub type ALARM = crate::interrupt::typelevel::RTC_ALARM;
        pub type STAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type TAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type WKUP = crate::interrupt::typelevel::RTC_WKUP;
    }
    pub mod SAI1 {
        pub type A = crate::interrupt::typelevel::SAI1;
        pub type B = crate::interrupt::typelevel::SAI1;
    }
    pub mod SDIO {
        pub type GLOBAL = crate::interrupt::typelevel::SDIO;
    }
    pub mod SPDIFRX1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPDIF_RX;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod SPI3 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI3;
    }
    pub mod SYSCFG {}
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM10;
    }
    pub mod TIM10 {
        pub type BRK = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type CC = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type COM = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type TRG = crate::interrupt::typelevel::TIM1_UP_TIM10;
        pub type UP = crate::interrupt::typelevel::TIM1_UP_TIM10;
    }
    pub mod TIM11 {
        pub type BRK = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type CC = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
        pub type UP = crate::interrupt::typelevel::TIM1_TRG_COM_TIM11;
    }
    pub mod TIM12 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type CC = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type COM = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type TRG = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type UP = crate::interrupt::typelevel::TIM8_BRK_TIM12;
    }
    pub mod TIM13 {
        pub type BRK = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type CC = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type COM = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type TRG = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type UP = crate::interrupt::typelevel::TIM8_UP_TIM13;
    }
    pub mod TIM14 {
        pub type BRK = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type CC = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type UP = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3;
        pub type CC = crate::interrupt::typelevel::TIM3;
        pub type COM = crate::interrupt::typelevel::TIM3;
        pub type TRG = crate::interrupt::typelevel::TIM3;
        pub type UP = crate::interrupt::typelevel::TIM3;
    }
    pub mod TIM4 {
        pub type BRK = crate::interrupt::typelevel::TIM4;
        pub type CC = crate::interrupt::typelevel::TIM4;
        pub type COM = crate::interrupt::typelevel::TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM4;
        pub type UP = crate::interrupt::typelevel::TIM4;
    }
    pub mod TIM5 {
        pub type BRK = crate::interrupt::typelevel::TIM5;
        pub type CC = crate::interrupt::typelevel::TIM5;
        pub type COM = crate::interrupt::typelevel::TIM5;
        pub type TRG = crate::interrupt::typelevel::TIM5;
        pub type UP = crate::interrupt::typelevel::TIM5;
    }
    pub mod TIM6 {
        pub type BRK = crate::interrupt::typelevel::TIM6_DAC;
        pub type CC = crate::interrupt::typelevel::TIM6_DAC;
        pub type COM = crate::interrupt::typelevel::TIM6_DAC;
        pub type TRG = crate::interrupt::typelevel::TIM6_DAC;
        pub type UP = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod TIM7 {
        pub type BRK = crate::interrupt::typelevel::TIM7;
        pub type CC = crate::interrupt::typelevel::TIM7;
        pub type COM = crate::interrupt::typelevel::TIM7;
        pub type TRG = crate::interrupt::typelevel::TIM7;
        pub type UP = crate::interrupt::typelevel::TIM7;
    }
    pub mod TIM8 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type CC = crate::interrupt::typelevel::TIM8_CC;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type UP = crate::interrupt::typelevel::TIM8_UP_TIM13;
    }
    pub mod TIM9 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type CC = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type COM = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type TRG = crate::interrupt::typelevel::TIM1_BRK_TIM9;
        pub type UP = crate::interrupt::typelevel::TIM1_BRK_TIM9;
    }
    pub mod UART4 {
        pub type GLOBAL = crate::interrupt::typelevel::UART4;
    }
    pub mod UART5 {
        pub type GLOBAL = crate::interrupt::typelevel::UART5;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2;
    }
    pub mod USART3 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3;
    }
    pub mod USART6 {
        pub type GLOBAL = crate::interrupt::typelevel::USART6;
    }
    pub mod USB_OTG_FS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_FS;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_FS;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_FS;
        pub type WKUP = crate::interrupt::typelevel::OTG_FS_WKUP;
    }
    pub mod USB_OTG_HS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_HS_EP1_IN;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_HS_EP1_OUT;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_HS;
        pub type WKUP = crate::interrupt::typelevel::OTG_HS_WKUP;
    }
    pub mod WWDG {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH0, crate::interrupt::typelevel::DMA1_STREAM0);
dma_channel_impl!(DMA1_CH1, crate::interrupt::typelevel::DMA1_STREAM1);
dma_channel_impl!(DMA1_CH2, crate::interrupt::typelevel::DMA1_STREAM2);
dma_channel_impl!(DMA1_CH3, crate::interrupt::typelevel::DMA1_STREAM3);
dma_channel_impl!(DMA1_CH4, crate::interrupt::typelevel::DMA1_STREAM4);
dma_channel_impl!(DMA1_CH5, crate::interrupt::typelevel::DMA1_STREAM5);
dma_channel_impl!(DMA1_CH6, crate::interrupt::typelevel::DMA1_STREAM6);
dma_channel_impl!(DMA1_CH7, crate::interrupt::typelevel::DMA1_STREAM7);
dma_channel_impl!(DMA2_CH0, crate::interrupt::typelevel::DMA2_STREAM0);
dma_channel_impl!(DMA2_CH1, crate::interrupt::typelevel::DMA2_STREAM1);
dma_channel_impl!(DMA2_CH2, crate::interrupt::typelevel::DMA2_STREAM2);
dma_channel_impl!(DMA2_CH3, crate::interrupt::typelevel::DMA2_STREAM3);
dma_channel_impl!(DMA2_CH4, crate::interrupt::typelevel::DMA2_STREAM4);
dma_channel_impl!(DMA2_CH5, crate::interrupt::typelevel::DMA2_STREAM5);
dma_channel_impl!(DMA2_CH6, crate::interrupt::typelevel::DMA2_STREAM6);
dma_channel_impl!(DMA2_CH7, crate::interrupt::typelevel::DMA2_STREAM7);
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 7usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 7usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
    },
];
#[derive(Copy, Clone)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub(crate) enum DmaChannel {
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH0,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7,
}
pub const fn gpio_block(port_num: usize) -> crate::pac::gpio::Gpio {
    #[cfg(stm32n6)]
    let port_num = if port_num > 7 { port_num + 5 } else { port_num };
    unsafe { crate::pac::gpio::Gpio::from_ptr((1073872896usize + 1024usize * port_num) as _) }
}
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 524288usize;
pub const WRITE_SIZE: usize = 4usize;
