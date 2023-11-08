# [doc = "Register `CTL1` reader"] pub type R = crate :: R < CTL1_SPEC > ; # [doc = "Register `CTL1` writer"] pub type W = crate :: W < CTL1_SPEC > ; # [doc = "Field `CTL1_ENABLE` reader - This bit turns on the comparator. When the comparator is turned off it consumes no power."] pub type CTL1_ENABLE_R = crate :: BitReader < CTL1_ENABLE_A > ; # [doc = "This bit turns on the comparator. When the comparator is turned off it consumes no power.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_ENABLE_A { # [doc = "0: OFF"] CTL1_ENABLE_OFF = 0 , # [doc = "1: ON"] CTL1_ENABLE_ON = 1 , } impl From < CTL1_ENABLE_A > for bool { # [inline (always)] fn from (variant : CTL1_ENABLE_A) -> Self { variant as u8 != 0 } } impl CTL1_ENABLE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_ENABLE_A { match self . bits { false => CTL1_ENABLE_A :: CTL1_ENABLE_OFF , true => CTL1_ENABLE_A :: CTL1_ENABLE_ON , } } # [doc = "OFF"] # [inline (always)] pub fn is_ctl1_enable_off (& self) -> bool { * self == CTL1_ENABLE_A :: CTL1_ENABLE_OFF } # [doc = "ON"] # [inline (always)] pub fn is_ctl1_enable_on (& self) -> bool { * self == CTL1_ENABLE_A :: CTL1_ENABLE_ON } } # [doc = "Field `CTL1_ENABLE` writer - This bit turns on the comparator. When the comparator is turned off it consumes no power."] pub type CTL1_ENABLE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_ENABLE_A > ; impl < 'a , REG , const O : u8 > CTL1_ENABLE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "OFF"] # [inline (always)] pub fn ctl1_enable_off (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_ENABLE_A :: CTL1_ENABLE_OFF) } # [doc = "ON"] # [inline (always)] pub fn ctl1_enable_on (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_ENABLE_A :: CTL1_ENABLE_ON) } } # [doc = "Field `CTL1_MODE` reader - This bit selects the comparator operating mode."] pub type CTL1_MODE_R = crate :: BitReader < CTL1_MODE_A > ; # [doc = "This bit selects the comparator operating mode.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_MODE_A { # [doc = "0: FAST"] CTL1_MODE_FAST = 0 , # [doc = "1: ULP"] CTL1_MODE_ULP = 1 , } impl From < CTL1_MODE_A > for bool { # [inline (always)] fn from (variant : CTL1_MODE_A) -> Self { variant as u8 != 0 } } impl CTL1_MODE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_MODE_A { match self . bits { false => CTL1_MODE_A :: CTL1_MODE_FAST , true => CTL1_MODE_A :: CTL1_MODE_ULP , } } # [doc = "FAST"] # [inline (always)] pub fn is_ctl1_mode_fast (& self) -> bool { * self == CTL1_MODE_A :: CTL1_MODE_FAST } # [doc = "ULP"] # [inline (always)] pub fn is_ctl1_mode_ulp (& self) -> bool { * self == CTL1_MODE_A :: CTL1_MODE_ULP } } # [doc = "Field `CTL1_MODE` writer - This bit selects the comparator operating mode."] pub type CTL1_MODE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_MODE_A > ; impl < 'a , REG , const O : u8 > CTL1_MODE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "FAST"] # [inline (always)] pub fn ctl1_mode_fast (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_MODE_A :: CTL1_MODE_FAST) } # [doc = "ULP"] # [inline (always)] pub fn ctl1_mode_ulp (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_MODE_A :: CTL1_MODE_ULP) } } # [doc = "Field `CTL1_EXCH` reader - This bit exchanges the comparator inputs and inverts the comparator output."] pub type CTL1_EXCH_R = crate :: BitReader < CTL1_EXCH_A > ; # [doc = "This bit exchanges the comparator inputs and inverts the comparator output.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_EXCH_A { # [doc = "0: NO_EXC"] CTL1_EXCH_NO_EXC = 0 , # [doc = "1: EXC"] CTL1_EXCH_EXC = 1 , } impl From < CTL1_EXCH_A > for bool { # [inline (always)] fn from (variant : CTL1_EXCH_A) -> Self { variant as u8 != 0 } } impl CTL1_EXCH_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_EXCH_A { match self . bits { false => CTL1_EXCH_A :: CTL1_EXCH_NO_EXC , true => CTL1_EXCH_A :: CTL1_EXCH_EXC , } } # [doc = "NO_EXC"] # [inline (always)] pub fn is_ctl1_exch_no_exc (& self) -> bool { * self == CTL1_EXCH_A :: CTL1_EXCH_NO_EXC } # [doc = "EXC"] # [inline (always)] pub fn is_ctl1_exch_exc (& self) -> bool { * self == CTL1_EXCH_A :: CTL1_EXCH_EXC } } # [doc = "Field `CTL1_EXCH` writer - This bit exchanges the comparator inputs and inverts the comparator output."] pub type CTL1_EXCH_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_EXCH_A > ; impl < 'a , REG , const O : u8 > CTL1_EXCH_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EXC"] # [inline (always)] pub fn ctl1_exch_no_exc (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_EXCH_A :: CTL1_EXCH_NO_EXC) } # [doc = "EXC"] # [inline (always)] pub fn ctl1_exch_exc (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_EXCH_A :: CTL1_EXCH_EXC) } } # [doc = "Field `CTL1_SHORT` reader - This bit shorts the positive and negative input terminals of the comparator."] pub type CTL1_SHORT_R = crate :: BitReader < CTL1_SHORT_A > ; # [doc = "This bit shorts the positive and negative input terminals of the comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_SHORT_A { # [doc = "0: NO_SHT"] CTL1_SHORT_NO_SHT = 0 , # [doc = "1: SHT"] CTL1_SHORT_SHT = 1 , } impl From < CTL1_SHORT_A > for bool { # [inline (always)] fn from (variant : CTL1_SHORT_A) -> Self { variant as u8 != 0 } } impl CTL1_SHORT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_SHORT_A { match self . bits { false => CTL1_SHORT_A :: CTL1_SHORT_NO_SHT , true => CTL1_SHORT_A :: CTL1_SHORT_SHT , } } # [doc = "NO_SHT"] # [inline (always)] pub fn is_ctl1_short_no_sht (& self) -> bool { * self == CTL1_SHORT_A :: CTL1_SHORT_NO_SHT } # [doc = "SHT"] # [inline (always)] pub fn is_ctl1_short_sht (& self) -> bool { * self == CTL1_SHORT_A :: CTL1_SHORT_SHT } } # [doc = "Field `CTL1_SHORT` writer - This bit shorts the positive and negative input terminals of the comparator."] pub type CTL1_SHORT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_SHORT_A > ; impl < 'a , REG , const O : u8 > CTL1_SHORT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_SHT"] # [inline (always)] pub fn ctl1_short_no_sht (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_SHORT_A :: CTL1_SHORT_NO_SHT) } # [doc = "SHT"] # [inline (always)] pub fn ctl1_short_sht (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_SHORT_A :: CTL1_SHORT_SHT) } } # [doc = "Field `CTL1_IES` reader - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."] pub type CTL1_IES_R = crate :: BitReader < CTL1_IES_A > ; # [doc = "This bit selected the interrupt edge for COMPIFG and COMPINVIFG.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_IES_A { # [doc = "0: RISING"] CTL1_IES_RISING = 0 , # [doc = "1: FALLING"] CTL1_IES_FALLING = 1 , } impl From < CTL1_IES_A > for bool { # [inline (always)] fn from (variant : CTL1_IES_A) -> Self { variant as u8 != 0 } } impl CTL1_IES_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_IES_A { match self . bits { false => CTL1_IES_A :: CTL1_IES_RISING , true => CTL1_IES_A :: CTL1_IES_FALLING , } } # [doc = "RISING"] # [inline (always)] pub fn is_ctl1_ies_rising (& self) -> bool { * self == CTL1_IES_A :: CTL1_IES_RISING } # [doc = "FALLING"] # [inline (always)] pub fn is_ctl1_ies_falling (& self) -> bool { * self == CTL1_IES_A :: CTL1_IES_FALLING } } # [doc = "Field `CTL1_IES` writer - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."] pub type CTL1_IES_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_IES_A > ; impl < 'a , REG , const O : u8 > CTL1_IES_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "RISING"] # [inline (always)] pub fn ctl1_ies_rising (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_IES_A :: CTL1_IES_RISING) } # [doc = "FALLING"] # [inline (always)] pub fn ctl1_ies_falling (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_IES_A :: CTL1_IES_FALLING) } } # [doc = "Field `CTL1_HYST` reader - These bits select the hysteresis setting of the comparator."] pub type CTL1_HYST_R = crate :: FieldReader < CTL1_HYST_A > ; # [doc = "These bits select the hysteresis setting of the comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CTL1_HYST_A { # [doc = "0: NO_HYS"] CTL1_HYST_NO_HYS = 0 , # [doc = "1: LOW_HYS"] CTL1_HYST_LOW_HYS = 1 , # [doc = "2: MED_HYS"] CTL1_HYST_MED_HYS = 2 , # [doc = "3: HIGH_HYS"] CTL1_HYST_HIGH_HYS = 3 , } impl From < CTL1_HYST_A > for u8 { # [inline (always)] fn from (variant : CTL1_HYST_A) -> Self { variant as _ } } impl crate :: FieldSpec for CTL1_HYST_A { type Ux = u8 ; } impl CTL1_HYST_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_HYST_A { match self . bits { 0 => CTL1_HYST_A :: CTL1_HYST_NO_HYS , 1 => CTL1_HYST_A :: CTL1_HYST_LOW_HYS , 2 => CTL1_HYST_A :: CTL1_HYST_MED_HYS , 3 => CTL1_HYST_A :: CTL1_HYST_HIGH_HYS , _ => unreachable ! () , } } # [doc = "NO_HYS"] # [inline (always)] pub fn is_ctl1_hyst_no_hys (& self) -> bool { * self == CTL1_HYST_A :: CTL1_HYST_NO_HYS } # [doc = "LOW_HYS"] # [inline (always)] pub fn is_ctl1_hyst_low_hys (& self) -> bool { * self == CTL1_HYST_A :: CTL1_HYST_LOW_HYS } # [doc = "MED_HYS"] # [inline (always)] pub fn is_ctl1_hyst_med_hys (& self) -> bool { * self == CTL1_HYST_A :: CTL1_HYST_MED_HYS } # [doc = "HIGH_HYS"] # [inline (always)] pub fn is_ctl1_hyst_high_hys (& self) -> bool { * self == CTL1_HYST_A :: CTL1_HYST_HIGH_HYS } } # [doc = "Field `CTL1_HYST` writer - These bits select the hysteresis setting of the comparator."] pub type CTL1_HYST_W < 'a , REG , const O : u8 > = crate :: FieldWriterSafe < 'a , REG , 2 , O , CTL1_HYST_A > ; impl < 'a , REG , const O : u8 > CTL1_HYST_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "NO_HYS"] # [inline (always)] pub fn ctl1_hyst_no_hys (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_HYST_A :: CTL1_HYST_NO_HYS) } # [doc = "LOW_HYS"] # [inline (always)] pub fn ctl1_hyst_low_hys (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_HYST_A :: CTL1_HYST_LOW_HYS) } # [doc = "MED_HYS"] # [inline (always)] pub fn ctl1_hyst_med_hys (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_HYST_A :: CTL1_HYST_MED_HYS) } # [doc = "HIGH_HYS"] # [inline (always)] pub fn ctl1_hyst_high_hys (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_HYST_A :: CTL1_HYST_HIGH_HYS) } } # [doc = "Field `CTL1_OUTPOL` reader - This bit selects the comparator output polarity."] pub type CTL1_OUTPOL_R = crate :: BitReader < CTL1_OUTPOL_A > ; # [doc = "This bit selects the comparator output polarity.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_OUTPOL_A { # [doc = "0: NON_INV"] CTL1_OUTPOL_NON_INV = 0 , # [doc = "1: INV"] CTL1_OUTPOL_INV = 1 , } impl From < CTL1_OUTPOL_A > for bool { # [inline (always)] fn from (variant : CTL1_OUTPOL_A) -> Self { variant as u8 != 0 } } impl CTL1_OUTPOL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_OUTPOL_A { match self . bits { false => CTL1_OUTPOL_A :: CTL1_OUTPOL_NON_INV , true => CTL1_OUTPOL_A :: CTL1_OUTPOL_INV , } } # [doc = "NON_INV"] # [inline (always)] pub fn is_ctl1_outpol_non_inv (& self) -> bool { * self == CTL1_OUTPOL_A :: CTL1_OUTPOL_NON_INV } # [doc = "INV"] # [inline (always)] pub fn is_ctl1_outpol_inv (& self) -> bool { * self == CTL1_OUTPOL_A :: CTL1_OUTPOL_INV } } # [doc = "Field `CTL1_OUTPOL` writer - This bit selects the comparator output polarity."] pub type CTL1_OUTPOL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_OUTPOL_A > ; impl < 'a , REG , const O : u8 > CTL1_OUTPOL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NON_INV"] # [inline (always)] pub fn ctl1_outpol_non_inv (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_OUTPOL_A :: CTL1_OUTPOL_NON_INV) } # [doc = "INV"] # [inline (always)] pub fn ctl1_outpol_inv (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_OUTPOL_A :: CTL1_OUTPOL_INV) } } # [doc = "Field `CTL1_FLTEN` reader - This bit enables the analog filter at comparator output."] pub type CTL1_FLTEN_R = crate :: BitReader < CTL1_FLTEN_A > ; # [doc = "This bit enables the analog filter at comparator output.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_FLTEN_A { # [doc = "0: DISABLE"] CTL1_FLTEN_DISABLE = 0 , # [doc = "1: ENABLE"] CTL1_FLTEN_ENABLE = 1 , } impl From < CTL1_FLTEN_A > for bool { # [inline (always)] fn from (variant : CTL1_FLTEN_A) -> Self { variant as u8 != 0 } } impl CTL1_FLTEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_FLTEN_A { match self . bits { false => CTL1_FLTEN_A :: CTL1_FLTEN_DISABLE , true => CTL1_FLTEN_A :: CTL1_FLTEN_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl1_flten_disable (& self) -> bool { * self == CTL1_FLTEN_A :: CTL1_FLTEN_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_ctl1_flten_enable (& self) -> bool { * self == CTL1_FLTEN_A :: CTL1_FLTEN_ENABLE } } # [doc = "Field `CTL1_FLTEN` writer - This bit enables the analog filter at comparator output."] pub type CTL1_FLTEN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_FLTEN_A > ; impl < 'a , REG , const O : u8 > CTL1_FLTEN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn ctl1_flten_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_FLTEN_A :: CTL1_FLTEN_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn ctl1_flten_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_FLTEN_A :: CTL1_FLTEN_ENABLE) } } # [doc = "Field `CTL1_FLTDLY` reader - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."] pub type CTL1_FLTDLY_R = crate :: FieldReader < CTL1_FLTDLY_A > ; # [doc = "These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CTL1_FLTDLY_A { # [doc = "0: DLY_0"] CTL1_FLTDLY_DLY_0 = 0 , # [doc = "1: DLY_1"] CTL1_FLTDLY_DLY_1 = 1 , # [doc = "2: DLY_2"] CTL1_FLTDLY_DLY_2 = 2 , # [doc = "3: DLY_3"] CTL1_FLTDLY_DLY_3 = 3 , } impl From < CTL1_FLTDLY_A > for u8 { # [inline (always)] fn from (variant : CTL1_FLTDLY_A) -> Self { variant as _ } } impl crate :: FieldSpec for CTL1_FLTDLY_A { type Ux = u8 ; } impl CTL1_FLTDLY_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_FLTDLY_A { match self . bits { 0 => CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_0 , 1 => CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_1 , 2 => CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_2 , 3 => CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_3 , _ => unreachable ! () , } } # [doc = "DLY_0"] # [inline (always)] pub fn is_ctl1_fltdly_dly_0 (& self) -> bool { * self == CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_0 } # [doc = "DLY_1"] # [inline (always)] pub fn is_ctl1_fltdly_dly_1 (& self) -> bool { * self == CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_1 } # [doc = "DLY_2"] # [inline (always)] pub fn is_ctl1_fltdly_dly_2 (& self) -> bool { * self == CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_2 } # [doc = "DLY_3"] # [inline (always)] pub fn is_ctl1_fltdly_dly_3 (& self) -> bool { * self == CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_3 } } # [doc = "Field `CTL1_FLTDLY` writer - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."] pub type CTL1_FLTDLY_W < 'a , REG , const O : u8 > = crate :: FieldWriterSafe < 'a , REG , 2 , O , CTL1_FLTDLY_A > ; impl < 'a , REG , const O : u8 > CTL1_FLTDLY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "DLY_0"] # [inline (always)] pub fn ctl1_fltdly_dly_0 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_0) } # [doc = "DLY_1"] # [inline (always)] pub fn ctl1_fltdly_dly_1 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_1) } # [doc = "DLY_2"] # [inline (always)] pub fn ctl1_fltdly_dly_2 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_2) } # [doc = "DLY_3"] # [inline (always)] pub fn ctl1_fltdly_dly_3 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_FLTDLY_A :: CTL1_FLTDLY_DLY_3) } } # [doc = "Field `CTL1_WINCOMPEN` reader - This bit enables window comparator operation of comparator."] pub type CTL1_WINCOMPEN_R = crate :: BitReader < CTL1_WINCOMPEN_A > ; # [doc = "This bit enables window comparator operation of comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL1_WINCOMPEN_A { # [doc = "0: OFF"] CTL1_WINCOMPEN_OFF = 0 , # [doc = "1: ON"] CTL1_WINCOMPEN_ON = 1 , } impl From < CTL1_WINCOMPEN_A > for bool { # [inline (always)] fn from (variant : CTL1_WINCOMPEN_A) -> Self { variant as u8 != 0 } } impl CTL1_WINCOMPEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL1_WINCOMPEN_A { match self . bits { false => CTL1_WINCOMPEN_A :: CTL1_WINCOMPEN_OFF , true => CTL1_WINCOMPEN_A :: CTL1_WINCOMPEN_ON , } } # [doc = "OFF"] # [inline (always)] pub fn is_ctl1_wincompen_off (& self) -> bool { * self == CTL1_WINCOMPEN_A :: CTL1_WINCOMPEN_OFF } # [doc = "ON"] # [inline (always)] pub fn is_ctl1_wincompen_on (& self) -> bool { * self == CTL1_WINCOMPEN_A :: CTL1_WINCOMPEN_ON } } # [doc = "Field `CTL1_WINCOMPEN` writer - This bit enables window comparator operation of comparator."] pub type CTL1_WINCOMPEN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL1_WINCOMPEN_A > ; impl < 'a , REG , const O : u8 > CTL1_WINCOMPEN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "OFF"] # [inline (always)] pub fn ctl1_wincompen_off (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_WINCOMPEN_A :: CTL1_WINCOMPEN_OFF) } # [doc = "ON"] # [inline (always)] pub fn ctl1_wincompen_on (self) -> & 'a mut crate :: W < REG > { self . variant (CTL1_WINCOMPEN_A :: CTL1_WINCOMPEN_ON) } } impl R { # [doc = "Bit 0 - This bit turns on the comparator. When the comparator is turned off it consumes no power."] # [inline (always)] pub fn ctl1_enable (& self) -> CTL1_ENABLE_R { CTL1_ENABLE_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - This bit selects the comparator operating mode."] # [inline (always)] pub fn ctl1_mode (& self) -> CTL1_MODE_R { CTL1_MODE_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - This bit exchanges the comparator inputs and inverts the comparator output."] # [inline (always)] pub fn ctl1_exch (& self) -> CTL1_EXCH_R { CTL1_EXCH_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - This bit shorts the positive and negative input terminals of the comparator."] # [inline (always)] pub fn ctl1_short (& self) -> CTL1_SHORT_R { CTL1_SHORT_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."] # [inline (always)] pub fn ctl1_ies (& self) -> CTL1_IES_R { CTL1_IES_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bits 5:6 - These bits select the hysteresis setting of the comparator."] # [inline (always)] pub fn ctl1_hyst (& self) -> CTL1_HYST_R { CTL1_HYST_R :: new (((self . bits >> 5) & 3) as u8) } # [doc = "Bit 7 - This bit selects the comparator output polarity."] # [inline (always)] pub fn ctl1_outpol (& self) -> CTL1_OUTPOL_R { CTL1_OUTPOL_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bit 8 - This bit enables the analog filter at comparator output."] # [inline (always)] pub fn ctl1_flten (& self) -> CTL1_FLTEN_R { CTL1_FLTEN_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bits 9:10 - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."] # [inline (always)] pub fn ctl1_fltdly (& self) -> CTL1_FLTDLY_R { CTL1_FLTDLY_R :: new (((self . bits >> 9) & 3) as u8) } # [doc = "Bit 12 - This bit enables window comparator operation of comparator."] # [inline (always)] pub fn ctl1_wincompen (& self) -> CTL1_WINCOMPEN_R { CTL1_WINCOMPEN_R :: new (((self . bits >> 12) & 1) != 0) } } impl W { # [doc = "Bit 0 - This bit turns on the comparator. When the comparator is turned off it consumes no power."] # [inline (always)] # [must_use] pub fn ctl1_enable (& mut self) -> CTL1_ENABLE_W < CTL1_SPEC , 0 > { CTL1_ENABLE_W :: new (self) } # [doc = "Bit 1 - This bit selects the comparator operating mode."] # [inline (always)] # [must_use] pub fn ctl1_mode (& mut self) -> CTL1_MODE_W < CTL1_SPEC , 1 > { CTL1_MODE_W :: new (self) } # [doc = "Bit 2 - This bit exchanges the comparator inputs and inverts the comparator output."] # [inline (always)] # [must_use] pub fn ctl1_exch (& mut self) -> CTL1_EXCH_W < CTL1_SPEC , 2 > { CTL1_EXCH_W :: new (self) } # [doc = "Bit 3 - This bit shorts the positive and negative input terminals of the comparator."] # [inline (always)] # [must_use] pub fn ctl1_short (& mut self) -> CTL1_SHORT_W < CTL1_SPEC , 3 > { CTL1_SHORT_W :: new (self) } # [doc = "Bit 4 - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."] # [inline (always)] # [must_use] pub fn ctl1_ies (& mut self) -> CTL1_IES_W < CTL1_SPEC , 4 > { CTL1_IES_W :: new (self) } # [doc = "Bits 5:6 - These bits select the hysteresis setting of the comparator."] # [inline (always)] # [must_use] pub fn ctl1_hyst (& mut self) -> CTL1_HYST_W < CTL1_SPEC , 5 > { CTL1_HYST_W :: new (self) } # [doc = "Bit 7 - This bit selects the comparator output polarity."] # [inline (always)] # [must_use] pub fn ctl1_outpol (& mut self) -> CTL1_OUTPOL_W < CTL1_SPEC , 7 > { CTL1_OUTPOL_W :: new (self) } # [doc = "Bit 8 - This bit enables the analog filter at comparator output."] # [inline (always)] # [must_use] pub fn ctl1_flten (& mut self) -> CTL1_FLTEN_W < CTL1_SPEC , 8 > { CTL1_FLTEN_W :: new (self) } # [doc = "Bits 9:10 - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."] # [inline (always)] # [must_use] pub fn ctl1_fltdly (& mut self) -> CTL1_FLTDLY_W < CTL1_SPEC , 9 > { CTL1_FLTDLY_W :: new (self) } # [doc = "Bit 12 - This bit enables window comparator operation of comparator."] # [inline (always)] # [must_use] pub fn ctl1_wincompen (& mut self) -> CTL1_WINCOMPEN_W < CTL1_SPEC , 12 > { CTL1_WINCOMPEN_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CTL1_SPEC ; impl crate :: RegisterSpec for CTL1_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ctl1::R`](R) reader structure"] impl crate :: Readable for CTL1_SPEC { } # [doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"] impl crate :: Writable for CTL1_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CTL1 to value 0"] impl crate :: Resettable for CTL1_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }