# [doc = "Register `IMASK` reader"] pub type R = crate :: R < IMASK_SPEC > ; # [doc = "Register `IMASK` writer"] pub type W = crate :: W < IMASK_SPEC > ; # [doc = "Field `IMASK_Z` reader - Zero Event mask"] pub type IMASK_Z_R = crate :: BitReader < IMASK_Z_A > ; # [doc = "Zero Event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_Z_A { # [doc = "0: CLR"] IMASK_Z_CLR = 0 , # [doc = "1: SET"] IMASK_Z_SET = 1 , } impl From < IMASK_Z_A > for bool { # [inline (always)] fn from (variant : IMASK_Z_A) -> Self { variant as u8 != 0 } } impl IMASK_Z_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_Z_A { match self . bits { false => IMASK_Z_A :: IMASK_Z_CLR , true => IMASK_Z_A :: IMASK_Z_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_z_clr (& self) -> bool { * self == IMASK_Z_A :: IMASK_Z_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_z_set (& self) -> bool { * self == IMASK_Z_A :: IMASK_Z_SET } } # [doc = "Field `IMASK_Z` writer - Zero Event mask"] pub type IMASK_Z_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_Z_A > ; impl < 'a , REG , const O : u8 > IMASK_Z_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_z_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_Z_A :: IMASK_Z_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_z_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_Z_A :: IMASK_Z_SET) } } # [doc = "Field `IMASK_L` reader - Load Event mask"] pub type IMASK_L_R = crate :: BitReader < IMASK_L_A > ; # [doc = "Load Event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_L_A { # [doc = "0: CLR"] IMASK_L_CLR = 0 , # [doc = "1: SET"] IMASK_L_SET = 1 , } impl From < IMASK_L_A > for bool { # [inline (always)] fn from (variant : IMASK_L_A) -> Self { variant as u8 != 0 } } impl IMASK_L_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_L_A { match self . bits { false => IMASK_L_A :: IMASK_L_CLR , true => IMASK_L_A :: IMASK_L_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_l_clr (& self) -> bool { * self == IMASK_L_A :: IMASK_L_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_l_set (& self) -> bool { * self == IMASK_L_A :: IMASK_L_SET } } # [doc = "Field `IMASK_L` writer - Load Event mask"] pub type IMASK_L_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_L_A > ; impl < 'a , REG , const O : u8 > IMASK_L_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_l_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_L_A :: IMASK_L_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_l_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_L_A :: IMASK_L_SET) } } # [doc = "Field `IMASK_CCD0` reader - Capture or Compare DN event mask CCP0"] pub type IMASK_CCD0_R = crate :: BitReader < IMASK_CCD0_A > ; # [doc = "Capture or Compare DN event mask CCP0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_CCD0_A { # [doc = "0: CLR"] IMASK_CCD0_CLR = 0 , # [doc = "1: SET"] IMASK_CCD0_SET = 1 , } impl From < IMASK_CCD0_A > for bool { # [inline (always)] fn from (variant : IMASK_CCD0_A) -> Self { variant as u8 != 0 } } impl IMASK_CCD0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_CCD0_A { match self . bits { false => IMASK_CCD0_A :: IMASK_CCD0_CLR , true => IMASK_CCD0_A :: IMASK_CCD0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_ccd0_clr (& self) -> bool { * self == IMASK_CCD0_A :: IMASK_CCD0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_ccd0_set (& self) -> bool { * self == IMASK_CCD0_A :: IMASK_CCD0_SET } } # [doc = "Field `IMASK_CCD0` writer - Capture or Compare DN event mask CCP0"] pub type IMASK_CCD0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_CCD0_A > ; impl < 'a , REG , const O : u8 > IMASK_CCD0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_ccd0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCD0_A :: IMASK_CCD0_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_ccd0_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCD0_A :: IMASK_CCD0_SET) } } # [doc = "Field `IMASK_CCD1` reader - Capture or Compare DN event mask CCP1"] pub type IMASK_CCD1_R = crate :: BitReader < IMASK_CCD1_A > ; # [doc = "Capture or Compare DN event mask CCP1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_CCD1_A { # [doc = "0: CLR"] IMASK_CCD1_CLR = 0 , # [doc = "1: SET"] IMASK_CCD1_SET = 1 , } impl From < IMASK_CCD1_A > for bool { # [inline (always)] fn from (variant : IMASK_CCD1_A) -> Self { variant as u8 != 0 } } impl IMASK_CCD1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_CCD1_A { match self . bits { false => IMASK_CCD1_A :: IMASK_CCD1_CLR , true => IMASK_CCD1_A :: IMASK_CCD1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_ccd1_clr (& self) -> bool { * self == IMASK_CCD1_A :: IMASK_CCD1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_ccd1_set (& self) -> bool { * self == IMASK_CCD1_A :: IMASK_CCD1_SET } } # [doc = "Field `IMASK_CCD1` writer - Capture or Compare DN event mask CCP1"] pub type IMASK_CCD1_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_CCD1_A > ; impl < 'a , REG , const O : u8 > IMASK_CCD1_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_ccd1_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCD1_A :: IMASK_CCD1_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_ccd1_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCD1_A :: IMASK_CCD1_SET) } } # [doc = "Field `IMASK_CCU0` reader - Capture or Compare UP event mask CCP0"] pub type IMASK_CCU0_R = crate :: BitReader < IMASK_CCU0_A > ; # [doc = "Capture or Compare UP event mask CCP0\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_CCU0_A { # [doc = "0: CLR"] IMASK_CCU0_CLR = 0 , # [doc = "1: SET"] IMASK_CCU0_SET = 1 , } impl From < IMASK_CCU0_A > for bool { # [inline (always)] fn from (variant : IMASK_CCU0_A) -> Self { variant as u8 != 0 } } impl IMASK_CCU0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_CCU0_A { match self . bits { false => IMASK_CCU0_A :: IMASK_CCU0_CLR , true => IMASK_CCU0_A :: IMASK_CCU0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_ccu0_clr (& self) -> bool { * self == IMASK_CCU0_A :: IMASK_CCU0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_ccu0_set (& self) -> bool { * self == IMASK_CCU0_A :: IMASK_CCU0_SET } } # [doc = "Field `IMASK_CCU0` writer - Capture or Compare UP event mask CCP0"] pub type IMASK_CCU0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_CCU0_A > ; impl < 'a , REG , const O : u8 > IMASK_CCU0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_ccu0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCU0_A :: IMASK_CCU0_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_ccu0_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCU0_A :: IMASK_CCU0_SET) } } # [doc = "Field `IMASK_CCU1` reader - Capture or Compare UP event mask CCP1"] pub type IMASK_CCU1_R = crate :: BitReader < IMASK_CCU1_A > ; # [doc = "Capture or Compare UP event mask CCP1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_CCU1_A { # [doc = "0: CLR"] IMASK_CCU1_CLR = 0 , # [doc = "1: SET"] IMASK_CCU1_SET = 1 , } impl From < IMASK_CCU1_A > for bool { # [inline (always)] fn from (variant : IMASK_CCU1_A) -> Self { variant as u8 != 0 } } impl IMASK_CCU1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_CCU1_A { match self . bits { false => IMASK_CCU1_A :: IMASK_CCU1_CLR , true => IMASK_CCU1_A :: IMASK_CCU1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_ccu1_clr (& self) -> bool { * self == IMASK_CCU1_A :: IMASK_CCU1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_ccu1_set (& self) -> bool { * self == IMASK_CCU1_A :: IMASK_CCU1_SET } } # [doc = "Field `IMASK_CCU1` writer - Capture or Compare UP event mask CCP1"] pub type IMASK_CCU1_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_CCU1_A > ; impl < 'a , REG , const O : u8 > IMASK_CCU1_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_ccu1_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCU1_A :: IMASK_CCU1_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_ccu1_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_CCU1_A :: IMASK_CCU1_SET) } } # [doc = "Field `IMASK_TOV` reader - Trigger Overflow Event mask"] pub type IMASK_TOV_R = crate :: BitReader < IMASK_TOV_A > ; # [doc = "Trigger Overflow Event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_TOV_A { # [doc = "0: CLR"] IMASK_TOV_CLR = 0 , # [doc = "1: SET"] IMASK_TOV_SET = 1 , } impl From < IMASK_TOV_A > for bool { # [inline (always)] fn from (variant : IMASK_TOV_A) -> Self { variant as u8 != 0 } } impl IMASK_TOV_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_TOV_A { match self . bits { false => IMASK_TOV_A :: IMASK_TOV_CLR , true => IMASK_TOV_A :: IMASK_TOV_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_tov_clr (& self) -> bool { * self == IMASK_TOV_A :: IMASK_TOV_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_tov_set (& self) -> bool { * self == IMASK_TOV_A :: IMASK_TOV_SET } } # [doc = "Field `IMASK_TOV` writer - Trigger Overflow Event mask"] pub type IMASK_TOV_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_TOV_A > ; impl < 'a , REG , const O : u8 > IMASK_TOV_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_tov_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_TOV_A :: IMASK_TOV_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_tov_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_TOV_A :: IMASK_TOV_SET) } } impl R { # [doc = "Bit 0 - Zero Event mask"] # [inline (always)] pub fn imask_z (& self) -> IMASK_Z_R { IMASK_Z_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Load Event mask"] # [inline (always)] pub fn imask_l (& self) -> IMASK_L_R { IMASK_L_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 4 - Capture or Compare DN event mask CCP0"] # [inline (always)] pub fn imask_ccd0 (& self) -> IMASK_CCD0_R { IMASK_CCD0_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Capture or Compare DN event mask CCP1"] # [inline (always)] pub fn imask_ccd1 (& self) -> IMASK_CCD1_R { IMASK_CCD1_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 8 - Capture or Compare UP event mask CCP0"] # [inline (always)] pub fn imask_ccu0 (& self) -> IMASK_CCU0_R { IMASK_CCU0_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Capture or Compare UP event mask CCP1"] # [inline (always)] pub fn imask_ccu1 (& self) -> IMASK_CCU1_R { IMASK_CCU1_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 25 - Trigger Overflow Event mask"] # [inline (always)] pub fn imask_tov (& self) -> IMASK_TOV_R { IMASK_TOV_R :: new (((self . bits >> 25) & 1) != 0) } } impl W { # [doc = "Bit 0 - Zero Event mask"] # [inline (always)] # [must_use] pub fn imask_z (& mut self) -> IMASK_Z_W < IMASK_SPEC , 0 > { IMASK_Z_W :: new (self) } # [doc = "Bit 1 - Load Event mask"] # [inline (always)] # [must_use] pub fn imask_l (& mut self) -> IMASK_L_W < IMASK_SPEC , 1 > { IMASK_L_W :: new (self) } # [doc = "Bit 4 - Capture or Compare DN event mask CCP0"] # [inline (always)] # [must_use] pub fn imask_ccd0 (& mut self) -> IMASK_CCD0_W < IMASK_SPEC , 4 > { IMASK_CCD0_W :: new (self) } # [doc = "Bit 5 - Capture or Compare DN event mask CCP1"] # [inline (always)] # [must_use] pub fn imask_ccd1 (& mut self) -> IMASK_CCD1_W < IMASK_SPEC , 5 > { IMASK_CCD1_W :: new (self) } # [doc = "Bit 8 - Capture or Compare UP event mask CCP0"] # [inline (always)] # [must_use] pub fn imask_ccu0 (& mut self) -> IMASK_CCU0_W < IMASK_SPEC , 8 > { IMASK_CCU0_W :: new (self) } # [doc = "Bit 9 - Capture or Compare UP event mask CCP1"] # [inline (always)] # [must_use] pub fn imask_ccu1 (& mut self) -> IMASK_CCU1_W < IMASK_SPEC , 9 > { IMASK_CCU1_W :: new (self) } # [doc = "Bit 25 - Trigger Overflow Event mask"] # [inline (always)] # [must_use] pub fn imask_tov (& mut self) -> IMASK_TOV_W < IMASK_SPEC , 25 > { IMASK_TOV_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IMASK_SPEC ; impl crate :: RegisterSpec for IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`imask::R`](R) reader structure"] impl crate :: Readable for IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"] impl crate :: Writable for IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets IMASK to value 0"] impl crate :: Resettable for IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }