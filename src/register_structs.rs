#![allow(clippy::missing_docs_in_private_items, clippy::no_effect_underscore_binding)]

use modular_bitfield::prelude::*;

// pub(crate) everywhere
// allow

pub(crate) trait RegisterWritable {
    fn into_reg_bytes(self) -> [u8; 3];
    fn from_reg_bytes(bytes: [u8; 3]) -> Self;
}

#[bitfield]
#[derive(Copy, Clone)]
pub(crate) struct R00h {
    #[skip] __: B20,
    sw_reset: bool,
    #[skip] __: B1,
    tm_count_rst: bool,
    reg_read: bool,
}

impl RegisterWritable for R00h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R01h {
    #[skip] _1: B8,
    led2stc: u16,
}

impl RegisterWritable for R01h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R02h {
    #[skip] _1: B8,
    led2endc: u16,
}

impl RegisterWritable for R02h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R03h {
    #[skip] _1: B8,
    led1ledstc: u16,
}

impl RegisterWritable for R03h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R04h {
    #[skip] _1: B8,
    led1ledendc: u16,
}

impl RegisterWritable for R04h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R05h {
    #[skip] _1: B8,
    aled2stc_or_led3stc: u16,
}

impl RegisterWritable for R05h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R06h {
    #[skip] _1: B8,
    aled2endc_or_led3endc: u16,
}

impl RegisterWritable for R06h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R07h {
    #[skip] _1: B8,
    led1stc: u16,
}

impl RegisterWritable for R07h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R08h {
    #[skip] _1: B8,
    led1endc: u16,
}

impl RegisterWritable for R08h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R09h {
    #[skip] _1: B8,
    led2ledstc: u16,
}

impl RegisterWritable for R09h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R0Ah {
    #[skip] _1: B8,
    led2ledendc: u16,
}

impl RegisterWritable for R0Ah {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R0Bh {
    #[skip] _1: B8,
    aled1stc: u16,
}

impl RegisterWritable for R0Bh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R0Ch {
    #[skip] _1: B8,
    aled1endc: u16,
}

impl RegisterWritable for R0Ch {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R0Dh {
    #[skip] _1: B8,
    led2convst: u16,
}

impl RegisterWritable for R0Dh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R0Eh {
    #[skip] _1: B8,
    led2convend: u16,
}

impl RegisterWritable for R0Eh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R0Fh {
    #[skip] _1: B8,
    aled2convst_or_led3convst: u16,
}

impl RegisterWritable for R0Fh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R10h {
    #[skip] _1: B8,
    aled2convend_or_led3convend: u16,
}

impl RegisterWritable for R10h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R11h {
    #[skip] _1: B8,
    led1convst: u16,
}

impl RegisterWritable for R11h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R12h {
    #[skip] _1: B8,
    led1convend: u16,
}

impl RegisterWritable for R12h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R13h {
    #[skip] _1: B8,
    aled1convst: u16,
}

impl RegisterWritable for R13h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R14h {
    #[skip] _1: B8,
    aled1convend: u16,
}

impl RegisterWritable for R14h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R15h {
    #[skip] _1: B8,
    adcrststct0: u16,
}

impl RegisterWritable for R15h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R16h {
    #[skip] _1: B8,
    adcrstendct0: u16,
}

impl RegisterWritable for R16h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R17h {
    #[skip] _1: B8,
    adcrststct1: u16,
}

impl RegisterWritable for R17h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R18h {
    #[skip] _1: B8,
    adcrstendct1: u16,
}

impl RegisterWritable for R18h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R19h {
    #[skip] _1: B8,
    adcrststct2: u16,
}

impl RegisterWritable for R19h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R1Ah {
    #[skip] _1: B8,
    adcrstendct2: u16,
}

impl RegisterWritable for R1Ah {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R1Bh {
    #[skip] _1: B8,
    adcrststct3: u16,
}

impl RegisterWritable for R1Bh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R1Ch {
    #[skip] _1: B8,
    adcrstendct3: u16,
}

impl RegisterWritable for R1Ch {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R1Dh {
    #[skip] _1: B8,
    prpct: u16,
}

impl RegisterWritable for R1Dh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R1Eh {
    #[skip] _1: B15,
    timeren: bool,
    #[skip] _2: B4,
    numav: B4,
}

impl RegisterWritable for R1Eh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R20h {
    #[skip] _1: B8,
    ensepgain: bool,
    #[skip] _2: B9,
    tia_cf_sep: B3,
    tia_gain_sep: B3,
}

impl RegisterWritable for R20h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R21h {
    #[skip] _1: B15,
    prog_tg_en: bool,
    #[skip] _2: B2,
    tia_cf: B3,
    tia_gain: B3,
}

impl RegisterWritable for R21h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R22h {
    #[skip] _1: B6,
    iled3: B6,
    iled2: B6,
    iled1: B6,
}

impl RegisterWritable for R22h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R23h {
    #[skip] _1: B3,
    dynamic1: bool,
    #[skip] _2: B2,
    iled_2x: bool,
    #[skip] _3: B2,
    dynamic2: bool,
    #[skip] _4: B4,
    osc_enable: bool,
    #[skip] _5: B4,
    dynamic3: bool,
    dynamic4: bool,
    #[skip] _6: B1,
    pdnrx: bool,
    pdnafe: bool,
}

impl RegisterWritable for R23h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R28h {
    #[skip] _1: B24,
}

impl RegisterWritable for R28h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R29h {
    #[skip] _1: B14,
    enable_clkout: bool,
    #[skip] _2: B4,
    clkdiv_clkout: B4,
    #[skip] _3: B1,
}

impl RegisterWritable for R29h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R2Ah {
    led2val: B24,
}

impl RegisterWritable for R2Ah {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R2Bh {
    aled2val_or_led3val: B24,
}

impl RegisterWritable for R2Bh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R2Ch {
    led1val: B24,
}

impl RegisterWritable for R2Ch {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R2Dh {
    aled1val: B24,
}

impl RegisterWritable for R2Dh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R2Eh {
    led2_minus_aled2val: B24,
}

impl RegisterWritable for R2Eh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R2Fh {
    led1_minus_aled1val: B24,
}

impl RegisterWritable for R2Fh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R31h {
    #[skip] _1: B13,
    pd_disconnect: bool,
    #[skip] _2: B4,
    enable_input_short: bool,
    #[skip] _3: B2,
    clkdiv_extmode: B3,
}

impl RegisterWritable for R31h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R32h {
    #[skip] _1: B8,
    pdncyclestc: u16,
}

impl RegisterWritable for R32h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R33h {
    #[skip] _1: B8,
    pdncycleendc: u16,
}

impl RegisterWritable for R33h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R34h {
    #[skip] _1: B8,
    prog_tg_stc: u16,
}

impl RegisterWritable for R34h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R35h {
    #[skip] _1: B8,
    prog_tg_endc: u16,
}

impl RegisterWritable for R35h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R36h {
    #[skip] _1: B8,
    led3ledstc: u16,
}

impl RegisterWritable for R36h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R37h {
    #[skip] _1: B8,
    led3ledendc: u16,
}

impl RegisterWritable for R37h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R39h {
    #[skip] _1: B21,
    clkdiv_prf: B3,
}

impl RegisterWritable for R39h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R3Ah {
    #[skip] _1: B4,
    pol_offdac_led2: bool,
    i_offdac_led2: B4,
    pol_offdac_amb1: bool,
    i_offdac_amb1: B4,
    pol_offdac_led1: bool,
    i_offdac_led1: B4,
    pol_offdac_amb2_or_pol_offdac_led3: bool,
    i_offdac_amb2_or_i_offdac_led3: B4,
}

impl RegisterWritable for R3Ah {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R3Dh {
    #[skip] _1: B18,
    dec_en: bool,
    #[skip] _2: B1,
    dec_factor: B3,
    #[skip] _3: B1,
}

impl RegisterWritable for R3Dh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R3Fh {
    avg_led2_minus_aled2val: B24,
}

impl RegisterWritable for R3Fh {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct R40h {
    avg_led1_minus_aled1val: B24,
}

impl RegisterWritable for R40h {
    fn into_reg_bytes(self) -> [u8; 3] {
        self.into_bytes()
    }

    fn from_reg_bytes(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

