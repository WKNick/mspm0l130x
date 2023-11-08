# [doc = "Register `SHUTDNSTORE1` reader"] pub type R = crate :: R < SHUTDNSTORE1_SPEC > ; # [doc = "Register `SHUTDNSTORE1` writer"] pub type W = crate :: W < SHUTDNSTORE1_SPEC > ; # [doc = "Field `SHUTDNSTORE1_DATA` reader - Shutdown storage byte 1"] pub type SHUTDNSTORE1_DATA_R = crate :: FieldReader ; # [doc = "Field `SHUTDNSTORE1_DATA` writer - Shutdown storage byte 1"] pub type SHUTDNSTORE1_DATA_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O > ; impl R { # [doc = "Bits 0:7 - Shutdown storage byte 1"] # [inline (always)] pub fn shutdnstore1_data (& self) -> SHUTDNSTORE1_DATA_R { SHUTDNSTORE1_DATA_R :: new ((self . bits & 0xff) as u8) } } impl W { # [doc = "Bits 0:7 - Shutdown storage byte 1"] # [inline (always)] # [must_use] pub fn shutdnstore1_data (& mut self) -> SHUTDNSTORE1_DATA_W < SHUTDNSTORE1_SPEC , 0 > { SHUTDNSTORE1_DATA_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Shutdown storage memory (byte 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shutdnstore1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shutdnstore1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct SHUTDNSTORE1_SPEC ; impl crate :: RegisterSpec for SHUTDNSTORE1_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`shutdnstore1::R`](R) reader structure"] impl crate :: Readable for SHUTDNSTORE1_SPEC { } # [doc = "`write(|w| ..)` method takes [`shutdnstore1::W`](W) writer structure"] impl crate :: Writable for SHUTDNSTORE1_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets SHUTDNSTORE1 to value 0"] impl crate :: Resettable for SHUTDNSTORE1_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }