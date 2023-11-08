# [doc = "Register `CMDWEPROTA` reader"] pub type R = crate :: R < CMDWEPROTA_SPEC > ; # [doc = "Register `CMDWEPROTA` writer"] pub type W = crate :: W < CMDWEPROTA_SPEC > ; # [doc = "Field `CMDWEPROTA_VAL` reader - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."] pub type CMDWEPROTA_VAL_R = crate :: FieldReader < u32 > ; # [doc = "Field `CMDWEPROTA_VAL` writer - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."] pub type CMDWEPROTA_VAL_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 32 , O , u32 > ; impl R { # [doc = "Bits 0:31 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."] # [inline (always)] pub fn cmdweprota_val (& self) -> CMDWEPROTA_VAL_R { CMDWEPROTA_VAL_R :: new (self . bits) } } impl W { # [doc = "Bits 0:31 - Each bit protects 1 sector. bit \\[0\\]: When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]: When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]: When 1, sector 31 of the flash memory will be protected from program and erase."] # [inline (always)] # [must_use] pub fn cmdweprota_val (& mut self) -> CMDWEPROTA_VAL_W < CMDWEPROTA_SPEC , 0 > { CMDWEPROTA_VAL_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Command Write Erase Protect A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprota::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprota::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CMDWEPROTA_SPEC ; impl crate :: RegisterSpec for CMDWEPROTA_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`cmdweprota::R`](R) reader structure"] impl crate :: Readable for CMDWEPROTA_SPEC { } # [doc = "`write(|w| ..)` method takes [`cmdweprota::W`](W) writer structure"] impl crate :: Writable for CMDWEPROTA_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CMDWEPROTA to value 0"] impl crate :: Resettable for CMDWEPROTA_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }