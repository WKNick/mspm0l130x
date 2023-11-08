# [doc = "Register `CFG` reader"] pub type R = crate :: R < CFG_SPEC > ; # [doc = "Register `CFG` writer"] pub type W = crate :: W < CFG_SPEC > ; # [doc = "Field `CFG_CHOP` reader - Chopping enable."] pub type CFG_CHOP_R = crate :: FieldReader < CFG_CHOP_A > ; # [doc = "Chopping enable.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CFG_CHOP_A { # [doc = "0: OFF"] CFG_CHOP_OFF = 0 , # [doc = "1: ON"] CFG_CHOP_ON = 1 , # [doc = "2: AVGON"] CFG_CHOP_AVGON = 2 , } impl From < CFG_CHOP_A > for u8 { # [inline (always)] fn from (variant : CFG_CHOP_A) -> Self { variant as _ } } impl crate :: FieldSpec for CFG_CHOP_A { type Ux = u8 ; } impl CFG_CHOP_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < CFG_CHOP_A > { match self . bits { 0 => Some (CFG_CHOP_A :: CFG_CHOP_OFF) , 1 => Some (CFG_CHOP_A :: CFG_CHOP_ON) , 2 => Some (CFG_CHOP_A :: CFG_CHOP_AVGON) , _ => None , } } # [doc = "OFF"] # [inline (always)] pub fn is_cfg_chop_off (& self) -> bool { * self == CFG_CHOP_A :: CFG_CHOP_OFF } # [doc = "ON"] # [inline (always)] pub fn is_cfg_chop_on (& self) -> bool { * self == CFG_CHOP_A :: CFG_CHOP_ON } # [doc = "AVGON"] # [inline (always)] pub fn is_cfg_chop_avgon (& self) -> bool { * self == CFG_CHOP_A :: CFG_CHOP_AVGON } } # [doc = "Field `CFG_CHOP` writer - Chopping enable."] pub type CFG_CHOP_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 2 , O , CFG_CHOP_A > ; impl < 'a , REG , const O : u8 > CFG_CHOP_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "OFF"] # [inline (always)] pub fn cfg_chop_off (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_CHOP_A :: CFG_CHOP_OFF) } # [doc = "ON"] # [inline (always)] pub fn cfg_chop_on (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_CHOP_A :: CFG_CHOP_ON) } # [doc = "AVGON"] # [inline (always)] pub fn cfg_chop_avgon (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_CHOP_A :: CFG_CHOP_AVGON) } } # [doc = "Field `CFG_OUTPIN` reader - Enable output pin"] pub type CFG_OUTPIN_R = crate :: BitReader < CFG_OUTPIN_A > ; # [doc = "Enable output pin\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CFG_OUTPIN_A { # [doc = "0: DISABLED"] CFG_OUTPIN_DISABLED = 0 , # [doc = "1: ENABLED"] CFG_OUTPIN_ENABLED = 1 , } impl From < CFG_OUTPIN_A > for bool { # [inline (always)] fn from (variant : CFG_OUTPIN_A) -> Self { variant as u8 != 0 } } impl CFG_OUTPIN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CFG_OUTPIN_A { match self . bits { false => CFG_OUTPIN_A :: CFG_OUTPIN_DISABLED , true => CFG_OUTPIN_A :: CFG_OUTPIN_ENABLED , } } # [doc = "DISABLED"] # [inline (always)] pub fn is_cfg_outpin_disabled (& self) -> bool { * self == CFG_OUTPIN_A :: CFG_OUTPIN_DISABLED } # [doc = "ENABLED"] # [inline (always)] pub fn is_cfg_outpin_enabled (& self) -> bool { * self == CFG_OUTPIN_A :: CFG_OUTPIN_ENABLED } } # [doc = "Field `CFG_OUTPIN` writer - Enable output pin"] pub type CFG_OUTPIN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CFG_OUTPIN_A > ; impl < 'a , REG , const O : u8 > CFG_OUTPIN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLED"] # [inline (always)] pub fn cfg_outpin_disabled (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_OUTPIN_A :: CFG_OUTPIN_DISABLED) } # [doc = "ENABLED"] # [inline (always)] pub fn cfg_outpin_enabled (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_OUTPIN_A :: CFG_OUTPIN_ENABLED) } } # [doc = "Field `CFG_PSEL` reader - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."] pub type CFG_PSEL_R = crate :: FieldReader < CFG_PSEL_A > ; # [doc = "Positive OA input selection. Please refer to the device specific datasheet for exact channels available.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CFG_PSEL_A { # [doc = "0: NC"] CFG_PSEL_NC = 0 , # [doc = "1: EXTPIN0"] CFG_PSEL_EXTPIN0 = 1 , # [doc = "2: EXTPIN1"] CFG_PSEL_EXTPIN1 = 2 , # [doc = "3: DAC12OUT"] CFG_PSEL_DAC12OUT = 3 , # [doc = "4: DAC8OUT"] CFG_PSEL_DAC8OUT = 4 , # [doc = "5: VREF"] CFG_PSEL_VREF = 5 , # [doc = "6: OANM1RTOP"] CFG_PSEL_OANM1RTOP = 6 , # [doc = "7: GPAMP_OUT_INT"] CFG_PSEL_GPAMP_OUT_INT = 7 , # [doc = "8: VSS"] CFG_PSEL_VSS = 8 , } impl From < CFG_PSEL_A > for u8 { # [inline (always)] fn from (variant : CFG_PSEL_A) -> Self { variant as _ } } impl crate :: FieldSpec for CFG_PSEL_A { type Ux = u8 ; } impl CFG_PSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < CFG_PSEL_A > { match self . bits { 0 => Some (CFG_PSEL_A :: CFG_PSEL_NC) , 1 => Some (CFG_PSEL_A :: CFG_PSEL_EXTPIN0) , 2 => Some (CFG_PSEL_A :: CFG_PSEL_EXTPIN1) , 3 => Some (CFG_PSEL_A :: CFG_PSEL_DAC12OUT) , 4 => Some (CFG_PSEL_A :: CFG_PSEL_DAC8OUT) , 5 => Some (CFG_PSEL_A :: CFG_PSEL_VREF) , 6 => Some (CFG_PSEL_A :: CFG_PSEL_OANM1RTOP) , 7 => Some (CFG_PSEL_A :: CFG_PSEL_GPAMP_OUT_INT) , 8 => Some (CFG_PSEL_A :: CFG_PSEL_VSS) , _ => None , } } # [doc = "NC"] # [inline (always)] pub fn is_cfg_psel_nc (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_NC } # [doc = "EXTPIN0"] # [inline (always)] pub fn is_cfg_psel_extpin0 (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_EXTPIN0 } # [doc = "EXTPIN1"] # [inline (always)] pub fn is_cfg_psel_extpin1 (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_EXTPIN1 } # [doc = "DAC12OUT"] # [inline (always)] pub fn is_cfg_psel_dac12out (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_DAC12OUT } # [doc = "DAC8OUT"] # [inline (always)] pub fn is_cfg_psel_dac8out (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_DAC8OUT } # [doc = "VREF"] # [inline (always)] pub fn is_cfg_psel_vref (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_VREF } # [doc = "OANM1RTOP"] # [inline (always)] pub fn is_cfg_psel_oanm1rtop (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_OANM1RTOP } # [doc = "GPAMP_OUT_INT"] # [inline (always)] pub fn is_cfg_psel_gpamp_out_int (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_GPAMP_OUT_INT } # [doc = "VSS"] # [inline (always)] pub fn is_cfg_psel_vss (& self) -> bool { * self == CFG_PSEL_A :: CFG_PSEL_VSS } } # [doc = "Field `CFG_PSEL` writer - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."] pub type CFG_PSEL_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 4 , O , CFG_PSEL_A > ; impl < 'a , REG , const O : u8 > CFG_PSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "NC"] # [inline (always)] pub fn cfg_psel_nc (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_NC) } # [doc = "EXTPIN0"] # [inline (always)] pub fn cfg_psel_extpin0 (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_EXTPIN0) } # [doc = "EXTPIN1"] # [inline (always)] pub fn cfg_psel_extpin1 (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_EXTPIN1) } # [doc = "DAC12OUT"] # [inline (always)] pub fn cfg_psel_dac12out (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_DAC12OUT) } # [doc = "DAC8OUT"] # [inline (always)] pub fn cfg_psel_dac8out (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_DAC8OUT) } # [doc = "VREF"] # [inline (always)] pub fn cfg_psel_vref (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_VREF) } # [doc = "OANM1RTOP"] # [inline (always)] pub fn cfg_psel_oanm1rtop (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_OANM1RTOP) } # [doc = "GPAMP_OUT_INT"] # [inline (always)] pub fn cfg_psel_gpamp_out_int (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_GPAMP_OUT_INT) } # [doc = "VSS"] # [inline (always)] pub fn cfg_psel_vss (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_PSEL_A :: CFG_PSEL_VSS) } } # [doc = "Field `CFG_NSEL` reader - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."] pub type CFG_NSEL_R = crate :: FieldReader < CFG_NSEL_A > ; # [doc = "Negative OA input selection. Please refer to the device specific datasheet for exact channels available.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CFG_NSEL_A { # [doc = "0: NC"] CFG_NSEL_NC = 0 , # [doc = "1: EXTPIN0"] CFG_NSEL_EXTPIN0 = 1 , # [doc = "2: EXTPIN1"] CFG_NSEL_EXTPIN1 = 2 , # [doc = "3: OANP1RBOT"] CFG_NSEL_OANP1RBOT = 3 , # [doc = "4: OANRTAP"] CFG_NSEL_OANRTAP = 4 , # [doc = "5: OANRTOP"] CFG_NSEL_OANRTOP = 5 , # [doc = "6: SPARE"] CFG_NSEL_SPARE = 6 , } impl From < CFG_NSEL_A > for u8 { # [inline (always)] fn from (variant : CFG_NSEL_A) -> Self { variant as _ } } impl crate :: FieldSpec for CFG_NSEL_A { type Ux = u8 ; } impl CFG_NSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < CFG_NSEL_A > { match self . bits { 0 => Some (CFG_NSEL_A :: CFG_NSEL_NC) , 1 => Some (CFG_NSEL_A :: CFG_NSEL_EXTPIN0) , 2 => Some (CFG_NSEL_A :: CFG_NSEL_EXTPIN1) , 3 => Some (CFG_NSEL_A :: CFG_NSEL_OANP1RBOT) , 4 => Some (CFG_NSEL_A :: CFG_NSEL_OANRTAP) , 5 => Some (CFG_NSEL_A :: CFG_NSEL_OANRTOP) , 6 => Some (CFG_NSEL_A :: CFG_NSEL_SPARE) , _ => None , } } # [doc = "NC"] # [inline (always)] pub fn is_cfg_nsel_nc (& self) -> bool { * self == CFG_NSEL_A :: CFG_NSEL_NC } # [doc = "EXTPIN0"] # [inline (always)] pub fn is_cfg_nsel_extpin0 (& self) -> bool { * self == CFG_NSEL_A :: CFG_NSEL_EXTPIN0 } # [doc = "EXTPIN1"] # [inline (always)] pub fn is_cfg_nsel_extpin1 (& self) -> bool { * self == CFG_NSEL_A :: CFG_NSEL_EXTPIN1 } # [doc = "OANP1RBOT"] # [inline (always)] pub fn is_cfg_nsel_oanp1rbot (& self) -> bool { * self == CFG_NSEL_A :: CFG_NSEL_OANP1RBOT } # [doc = "OANRTAP"] # [inline (always)] pub fn is_cfg_nsel_oanrtap (& self) -> bool { * self == CFG_NSEL_A :: CFG_NSEL_OANRTAP } # [doc = "OANRTOP"] # [inline (always)] pub fn is_cfg_nsel_oanrtop (& self) -> bool { * self == CFG_NSEL_A :: CFG_NSEL_OANRTOP } # [doc = "SPARE"] # [inline (always)] pub fn is_cfg_nsel_spare (& self) -> bool { * self == CFG_NSEL_A :: CFG_NSEL_SPARE } } # [doc = "Field `CFG_NSEL` writer - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."] pub type CFG_NSEL_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 3 , O , CFG_NSEL_A > ; impl < 'a , REG , const O : u8 > CFG_NSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "NC"] # [inline (always)] pub fn cfg_nsel_nc (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_NSEL_A :: CFG_NSEL_NC) } # [doc = "EXTPIN0"] # [inline (always)] pub fn cfg_nsel_extpin0 (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_NSEL_A :: CFG_NSEL_EXTPIN0) } # [doc = "EXTPIN1"] # [inline (always)] pub fn cfg_nsel_extpin1 (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_NSEL_A :: CFG_NSEL_EXTPIN1) } # [doc = "OANP1RBOT"] # [inline (always)] pub fn cfg_nsel_oanp1rbot (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_NSEL_A :: CFG_NSEL_OANP1RBOT) } # [doc = "OANRTAP"] # [inline (always)] pub fn cfg_nsel_oanrtap (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_NSEL_A :: CFG_NSEL_OANRTAP) } # [doc = "OANRTOP"] # [inline (always)] pub fn cfg_nsel_oanrtop (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_NSEL_A :: CFG_NSEL_OANRTOP) } # [doc = "SPARE"] # [inline (always)] pub fn cfg_nsel_spare (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_NSEL_A :: CFG_NSEL_SPARE) } } # [doc = "Field `CFG_MSEL` reader - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."] pub type CFG_MSEL_R = crate :: FieldReader < CFG_MSEL_A > ; # [doc = "MSEL Mux selection. Please refer to the device specific datasheet for exact channels available.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CFG_MSEL_A { # [doc = "0: NC"] CFG_MSEL_NC = 0 , # [doc = "1: EXTNPIN1"] CFG_MSEL_EXTNPIN1 = 1 , # [doc = "2: VSS"] CFG_MSEL_VSS = 2 , # [doc = "3: DAC12OUT"] CFG_MSEL_DAC12OUT = 3 , # [doc = "4: OANM1RTOP"] CFG_MSEL_OANM1RTOP = 4 , } impl From < CFG_MSEL_A > for u8 { # [inline (always)] fn from (variant : CFG_MSEL_A) -> Self { variant as _ } } impl crate :: FieldSpec for CFG_MSEL_A { type Ux = u8 ; } impl CFG_MSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < CFG_MSEL_A > { match self . bits { 0 => Some (CFG_MSEL_A :: CFG_MSEL_NC) , 1 => Some (CFG_MSEL_A :: CFG_MSEL_EXTNPIN1) , 2 => Some (CFG_MSEL_A :: CFG_MSEL_VSS) , 3 => Some (CFG_MSEL_A :: CFG_MSEL_DAC12OUT) , 4 => Some (CFG_MSEL_A :: CFG_MSEL_OANM1RTOP) , _ => None , } } # [doc = "NC"] # [inline (always)] pub fn is_cfg_msel_nc (& self) -> bool { * self == CFG_MSEL_A :: CFG_MSEL_NC } # [doc = "EXTNPIN1"] # [inline (always)] pub fn is_cfg_msel_extnpin1 (& self) -> bool { * self == CFG_MSEL_A :: CFG_MSEL_EXTNPIN1 } # [doc = "VSS"] # [inline (always)] pub fn is_cfg_msel_vss (& self) -> bool { * self == CFG_MSEL_A :: CFG_MSEL_VSS } # [doc = "DAC12OUT"] # [inline (always)] pub fn is_cfg_msel_dac12out (& self) -> bool { * self == CFG_MSEL_A :: CFG_MSEL_DAC12OUT } # [doc = "OANM1RTOP"] # [inline (always)] pub fn is_cfg_msel_oanm1rtop (& self) -> bool { * self == CFG_MSEL_A :: CFG_MSEL_OANM1RTOP } } # [doc = "Field `CFG_MSEL` writer - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."] pub type CFG_MSEL_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 3 , O , CFG_MSEL_A > ; impl < 'a , REG , const O : u8 > CFG_MSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "NC"] # [inline (always)] pub fn cfg_msel_nc (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_MSEL_A :: CFG_MSEL_NC) } # [doc = "EXTNPIN1"] # [inline (always)] pub fn cfg_msel_extnpin1 (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_MSEL_A :: CFG_MSEL_EXTNPIN1) } # [doc = "VSS"] # [inline (always)] pub fn cfg_msel_vss (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_MSEL_A :: CFG_MSEL_VSS) } # [doc = "DAC12OUT"] # [inline (always)] pub fn cfg_msel_dac12out (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_MSEL_A :: CFG_MSEL_DAC12OUT) } # [doc = "OANM1RTOP"] # [inline (always)] pub fn cfg_msel_oanm1rtop (self) -> & 'a mut crate :: W < REG > { self . variant (CFG_MSEL_A :: CFG_MSEL_OANM1RTOP) } } # [doc = "Field `CFG_GAIN` reader - Gain setting. Refer to TRM for enumeration information."] pub type CFG_GAIN_R = crate :: FieldReader ; # [doc = "Field `CFG_GAIN` writer - Gain setting. Refer to TRM for enumeration information."] pub type CFG_GAIN_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 3 , O > ; impl R { # [doc = "Bits 0:1 - Chopping enable."] # [inline (always)] pub fn cfg_chop (& self) -> CFG_CHOP_R { CFG_CHOP_R :: new ((self . bits & 3) as u8) } # [doc = "Bit 2 - Enable output pin"] # [inline (always)] pub fn cfg_outpin (& self) -> CFG_OUTPIN_R { CFG_OUTPIN_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bits 3:6 - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."] # [inline (always)] pub fn cfg_psel (& self) -> CFG_PSEL_R { CFG_PSEL_R :: new (((self . bits >> 3) & 0x0f) as u8) } # [doc = "Bits 7:9 - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."] # [inline (always)] pub fn cfg_nsel (& self) -> CFG_NSEL_R { CFG_NSEL_R :: new (((self . bits >> 7) & 7) as u8) } # [doc = "Bits 10:12 - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."] # [inline (always)] pub fn cfg_msel (& self) -> CFG_MSEL_R { CFG_MSEL_R :: new (((self . bits >> 10) & 7) as u8) } # [doc = "Bits 13:15 - Gain setting. Refer to TRM for enumeration information."] # [inline (always)] pub fn cfg_gain (& self) -> CFG_GAIN_R { CFG_GAIN_R :: new (((self . bits >> 13) & 7) as u8) } } impl W { # [doc = "Bits 0:1 - Chopping enable."] # [inline (always)] # [must_use] pub fn cfg_chop (& mut self) -> CFG_CHOP_W < CFG_SPEC , 0 > { CFG_CHOP_W :: new (self) } # [doc = "Bit 2 - Enable output pin"] # [inline (always)] # [must_use] pub fn cfg_outpin (& mut self) -> CFG_OUTPIN_W < CFG_SPEC , 2 > { CFG_OUTPIN_W :: new (self) } # [doc = "Bits 3:6 - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."] # [inline (always)] # [must_use] pub fn cfg_psel (& mut self) -> CFG_PSEL_W < CFG_SPEC , 3 > { CFG_PSEL_W :: new (self) } # [doc = "Bits 7:9 - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."] # [inline (always)] # [must_use] pub fn cfg_nsel (& mut self) -> CFG_NSEL_W < CFG_SPEC , 7 > { CFG_NSEL_W :: new (self) } # [doc = "Bits 10:12 - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."] # [inline (always)] # [must_use] pub fn cfg_msel (& mut self) -> CFG_MSEL_W < CFG_SPEC , 10 > { CFG_MSEL_W :: new (self) } # [doc = "Bits 13:15 - Gain setting. Refer to TRM for enumeration information."] # [inline (always)] # [must_use] pub fn cfg_gain (& mut self) -> CFG_GAIN_W < CFG_SPEC , 13 > { CFG_GAIN_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CFG_SPEC ; impl crate :: RegisterSpec for CFG_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`cfg::R`](R) reader structure"] impl crate :: Readable for CFG_SPEC { } # [doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"] impl crate :: Writable for CFG_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CFG to value 0"] impl crate :: Resettable for CFG_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }