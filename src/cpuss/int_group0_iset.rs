# [doc = "Register `INT_GROUP0_ISET` writer"] pub type W = crate :: W < INT_GROUP0_ISET_SPEC > ; # [doc = "Sets INT in RIS register\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_GROUP0_ISET_INT_AW { # [doc = "0: NO_EFFECT"] INT_GROUP0_ISET_INT_NO_EFFECT = 0 , # [doc = "1: SET"] INT_GROUP0_ISET_INT_SET = 1 , } impl From < INT_GROUP0_ISET_INT_AW > for bool { # [inline (always)] fn from (variant : INT_GROUP0_ISET_INT_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_GROUP0_ISET_INT` writer - Sets INT in RIS register"] pub type INT_GROUP0_ISET_INT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_GROUP0_ISET_INT_AW > ; impl < 'a , REG , const O : u8 > INT_GROUP0_ISET_INT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_group0_iset_int_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_GROUP0_ISET_INT_AW :: INT_GROUP0_ISET_INT_NO_EFFECT) } # [doc = "SET"] # [inline (always)] pub fn int_group0_iset_int_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_GROUP0_ISET_INT_AW :: INT_GROUP0_ISET_INT_SET) } } impl W { # [doc = "Bit 0 - Sets INT in RIS register"] # [inline (always)] # [must_use] pub fn int_group0_iset_int (& mut self) -> INT_GROUP0_ISET_INT_W < INT_GROUP0_ISET_SPEC , 0 > { INT_GROUP0_ISET_INT_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_group0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_GROUP0_ISET_SPEC ; impl crate :: RegisterSpec for INT_GROUP0_ISET_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`int_group0_iset::W`](W) writer structure"] impl crate :: Writable for INT_GROUP0_ISET_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_GROUP0_ISET to value 0"] impl crate :: Resettable for INT_GROUP0_ISET_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }