# [doc = "Register `CTL0` reader"] pub type R = crate :: R < CTL0_SPEC > ; # [doc = "Register `CTL0` writer"] pub type W = crate :: W < CTL0_SPEC > ; # [doc = "Field `CTL0_IPSEL` reader - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."] pub type CTL0_IPSEL_R = crate :: FieldReader < CTL0_IPSEL_A > ; # [doc = "Channel input selected for the positive terminal of the comparator if IPEN is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CTL0_IPSEL_A { # [doc = "0: CH_0"] CTL0_IPSEL_CH_0 = 0 , # [doc = "1: CH_1"] CTL0_IPSEL_CH_1 = 1 , # [doc = "2: CH_2"] CTL0_IPSEL_CH_2 = 2 , # [doc = "3: CH_3"] CTL0_IPSEL_CH_3 = 3 , # [doc = "4: CH_4"] CTL0_IPSEL_CH_4 = 4 , # [doc = "5: CH_5"] CTL0_IPSEL_CH_5 = 5 , # [doc = "6: CH_6"] CTL0_IPSEL_CH_6 = 6 , # [doc = "7: CH_7"] CTL0_IPSEL_CH_7 = 7 , } impl From < CTL0_IPSEL_A > for u8 { # [inline (always)] fn from (variant : CTL0_IPSEL_A) -> Self { variant as _ } } impl crate :: FieldSpec for CTL0_IPSEL_A { type Ux = u8 ; } impl CTL0_IPSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL0_IPSEL_A { match self . bits { 0 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_0 , 1 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_1 , 2 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_2 , 3 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_3 , 4 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_4 , 5 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_5 , 6 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_6 , 7 => CTL0_IPSEL_A :: CTL0_IPSEL_CH_7 , _ => unreachable ! () , } } # [doc = "CH_0"] # [inline (always)] pub fn is_ctl0_ipsel_ch_0 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_0 } # [doc = "CH_1"] # [inline (always)] pub fn is_ctl0_ipsel_ch_1 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_1 } # [doc = "CH_2"] # [inline (always)] pub fn is_ctl0_ipsel_ch_2 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_2 } # [doc = "CH_3"] # [inline (always)] pub fn is_ctl0_ipsel_ch_3 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_3 } # [doc = "CH_4"] # [inline (always)] pub fn is_ctl0_ipsel_ch_4 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_4 } # [doc = "CH_5"] # [inline (always)] pub fn is_ctl0_ipsel_ch_5 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_5 } # [doc = "CH_6"] # [inline (always)] pub fn is_ctl0_ipsel_ch_6 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_6 } # [doc = "CH_7"] # [inline (always)] pub fn is_ctl0_ipsel_ch_7 (& self) -> bool { * self == CTL0_IPSEL_A :: CTL0_IPSEL_CH_7 } } # [doc = "Field `CTL0_IPSEL` writer - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."] pub type CTL0_IPSEL_W < 'a , REG , const O : u8 > = crate :: FieldWriterSafe < 'a , REG , 3 , O , CTL0_IPSEL_A > ; impl < 'a , REG , const O : u8 > CTL0_IPSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "CH_0"] # [inline (always)] pub fn ctl0_ipsel_ch_0 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_0) } # [doc = "CH_1"] # [inline (always)] pub fn ctl0_ipsel_ch_1 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_1) } # [doc = "CH_2"] # [inline (always)] pub fn ctl0_ipsel_ch_2 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_2) } # [doc = "CH_3"] # [inline (always)] pub fn ctl0_ipsel_ch_3 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_3) } # [doc = "CH_4"] # [inline (always)] pub fn ctl0_ipsel_ch_4 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_4) } # [doc = "CH_5"] # [inline (always)] pub fn ctl0_ipsel_ch_5 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_5) } # [doc = "CH_6"] # [inline (always)] pub fn ctl0_ipsel_ch_6 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_6) } # [doc = "CH_7"] # [inline (always)] pub fn ctl0_ipsel_ch_7 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPSEL_A :: CTL0_IPSEL_CH_7) } } # [doc = "Field `CTL0_IPEN` reader - Channel input enable for the positive terminal of the comparator."] pub type CTL0_IPEN_R = crate :: BitReader < CTL0_IPEN_A > ; # [doc = "Channel input enable for the positive terminal of the comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL0_IPEN_A { # [doc = "0: DISABLE"] CTL0_IPEN_DISABLE = 0 , # [doc = "1: ENABLE"] CTL0_IPEN_ENABLE = 1 , } impl From < CTL0_IPEN_A > for bool { # [inline (always)] fn from (variant : CTL0_IPEN_A) -> Self { variant as u8 != 0 } } impl CTL0_IPEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL0_IPEN_A { match self . bits { false => CTL0_IPEN_A :: CTL0_IPEN_DISABLE , true => CTL0_IPEN_A :: CTL0_IPEN_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl0_ipen_disable (& self) -> bool { * self == CTL0_IPEN_A :: CTL0_IPEN_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_ctl0_ipen_enable (& self) -> bool { * self == CTL0_IPEN_A :: CTL0_IPEN_ENABLE } } # [doc = "Field `CTL0_IPEN` writer - Channel input enable for the positive terminal of the comparator."] pub type CTL0_IPEN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL0_IPEN_A > ; impl < 'a , REG , const O : u8 > CTL0_IPEN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn ctl0_ipen_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPEN_A :: CTL0_IPEN_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn ctl0_ipen_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IPEN_A :: CTL0_IPEN_ENABLE) } } # [doc = "Field `CTL0_IMSEL` reader - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."] pub type CTL0_IMSEL_R = crate :: FieldReader < CTL0_IMSEL_A > ; # [doc = "Channel input selected for the negative terminal of the comparator if IMEN is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CTL0_IMSEL_A { # [doc = "0: CH_0"] CTL0_IMSEL_CH_0 = 0 , # [doc = "1: CH_1"] CTL0_IMSEL_CH_1 = 1 , # [doc = "2: CH_2"] CTL0_IMSEL_CH_2 = 2 , # [doc = "3: CH_3"] CTL0_IMSEL_CH_3 = 3 , # [doc = "4: CH_4"] CTL0_IMSEL_CH_4 = 4 , # [doc = "5: CH_5"] CTL0_IMSEL_CH_5 = 5 , # [doc = "6: CH_6"] CTL0_IMSEL_CH_6 = 6 , # [doc = "7: CH_7"] CTL0_IMSEL_CH_7 = 7 , } impl From < CTL0_IMSEL_A > for u8 { # [inline (always)] fn from (variant : CTL0_IMSEL_A) -> Self { variant as _ } } impl crate :: FieldSpec for CTL0_IMSEL_A { type Ux = u8 ; } impl CTL0_IMSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL0_IMSEL_A { match self . bits { 0 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_0 , 1 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_1 , 2 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_2 , 3 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_3 , 4 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_4 , 5 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_5 , 6 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_6 , 7 => CTL0_IMSEL_A :: CTL0_IMSEL_CH_7 , _ => unreachable ! () , } } # [doc = "CH_0"] # [inline (always)] pub fn is_ctl0_imsel_ch_0 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_0 } # [doc = "CH_1"] # [inline (always)] pub fn is_ctl0_imsel_ch_1 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_1 } # [doc = "CH_2"] # [inline (always)] pub fn is_ctl0_imsel_ch_2 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_2 } # [doc = "CH_3"] # [inline (always)] pub fn is_ctl0_imsel_ch_3 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_3 } # [doc = "CH_4"] # [inline (always)] pub fn is_ctl0_imsel_ch_4 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_4 } # [doc = "CH_5"] # [inline (always)] pub fn is_ctl0_imsel_ch_5 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_5 } # [doc = "CH_6"] # [inline (always)] pub fn is_ctl0_imsel_ch_6 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_6 } # [doc = "CH_7"] # [inline (always)] pub fn is_ctl0_imsel_ch_7 (& self) -> bool { * self == CTL0_IMSEL_A :: CTL0_IMSEL_CH_7 } } # [doc = "Field `CTL0_IMSEL` writer - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."] pub type CTL0_IMSEL_W < 'a , REG , const O : u8 > = crate :: FieldWriterSafe < 'a , REG , 3 , O , CTL0_IMSEL_A > ; impl < 'a , REG , const O : u8 > CTL0_IMSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "CH_0"] # [inline (always)] pub fn ctl0_imsel_ch_0 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_0) } # [doc = "CH_1"] # [inline (always)] pub fn ctl0_imsel_ch_1 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_1) } # [doc = "CH_2"] # [inline (always)] pub fn ctl0_imsel_ch_2 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_2) } # [doc = "CH_3"] # [inline (always)] pub fn ctl0_imsel_ch_3 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_3) } # [doc = "CH_4"] # [inline (always)] pub fn ctl0_imsel_ch_4 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_4) } # [doc = "CH_5"] # [inline (always)] pub fn ctl0_imsel_ch_5 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_5) } # [doc = "CH_6"] # [inline (always)] pub fn ctl0_imsel_ch_6 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_6) } # [doc = "CH_7"] # [inline (always)] pub fn ctl0_imsel_ch_7 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMSEL_A :: CTL0_IMSEL_CH_7) } } # [doc = "Field `CTL0_IMEN` reader - Channel input enable for the negative terminal of the comparator."] pub type CTL0_IMEN_R = crate :: BitReader < CTL0_IMEN_A > ; # [doc = "Channel input enable for the negative terminal of the comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL0_IMEN_A { # [doc = "0: DISABLE"] CTL0_IMEN_DISABLE = 0 , # [doc = "1: ENABLE"] CTL0_IMEN_ENABLE = 1 , } impl From < CTL0_IMEN_A > for bool { # [inline (always)] fn from (variant : CTL0_IMEN_A) -> Self { variant as u8 != 0 } } impl CTL0_IMEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL0_IMEN_A { match self . bits { false => CTL0_IMEN_A :: CTL0_IMEN_DISABLE , true => CTL0_IMEN_A :: CTL0_IMEN_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl0_imen_disable (& self) -> bool { * self == CTL0_IMEN_A :: CTL0_IMEN_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_ctl0_imen_enable (& self) -> bool { * self == CTL0_IMEN_A :: CTL0_IMEN_ENABLE } } # [doc = "Field `CTL0_IMEN` writer - Channel input enable for the negative terminal of the comparator."] pub type CTL0_IMEN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL0_IMEN_A > ; impl < 'a , REG , const O : u8 > CTL0_IMEN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn ctl0_imen_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMEN_A :: CTL0_IMEN_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn ctl0_imen_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL0_IMEN_A :: CTL0_IMEN_ENABLE) } } impl R { # [doc = "Bits 0:2 - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."] # [inline (always)] pub fn ctl0_ipsel (& self) -> CTL0_IPSEL_R { CTL0_IPSEL_R :: new ((self . bits & 7) as u8) } # [doc = "Bit 15 - Channel input enable for the positive terminal of the comparator."] # [inline (always)] pub fn ctl0_ipen (& self) -> CTL0_IPEN_R { CTL0_IPEN_R :: new (((self . bits >> 15) & 1) != 0) } # [doc = "Bits 16:18 - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."] # [inline (always)] pub fn ctl0_imsel (& self) -> CTL0_IMSEL_R { CTL0_IMSEL_R :: new (((self . bits >> 16) & 7) as u8) } # [doc = "Bit 31 - Channel input enable for the negative terminal of the comparator."] # [inline (always)] pub fn ctl0_imen (& self) -> CTL0_IMEN_R { CTL0_IMEN_R :: new (((self . bits >> 31) & 1) != 0) } } impl W { # [doc = "Bits 0:2 - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."] # [inline (always)] # [must_use] pub fn ctl0_ipsel (& mut self) -> CTL0_IPSEL_W < CTL0_SPEC , 0 > { CTL0_IPSEL_W :: new (self) } # [doc = "Bit 15 - Channel input enable for the positive terminal of the comparator."] # [inline (always)] # [must_use] pub fn ctl0_ipen (& mut self) -> CTL0_IPEN_W < CTL0_SPEC , 15 > { CTL0_IPEN_W :: new (self) } # [doc = "Bits 16:18 - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."] # [inline (always)] # [must_use] pub fn ctl0_imsel (& mut self) -> CTL0_IMSEL_W < CTL0_SPEC , 16 > { CTL0_IMSEL_W :: new (self) } # [doc = "Bit 31 - Channel input enable for the negative terminal of the comparator."] # [inline (always)] # [must_use] pub fn ctl0_imen (& mut self) -> CTL0_IMEN_W < CTL0_SPEC , 31 > { CTL0_IMEN_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CTL0_SPEC ; impl crate :: RegisterSpec for CTL0_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ctl0::R`](R) reader structure"] impl crate :: Readable for CTL0_SPEC { } # [doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"] impl crate :: Writable for CTL0_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CTL0 to value 0"] impl crate :: Resettable for CTL0_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }