# [doc = "Register `INT_EVENT0_IMASK` reader"] pub type R = crate :: R < INT_EVENT0_IMASK_SPEC > ; # [doc = "Register `INT_EVENT0_IMASK` writer"] pub type W = crate :: W < INT_EVENT0_IMASK_SPEC > ; # [doc = "Field `INT_EVENT0_IMASK_OVIFG` reader - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_OVIFG_R = crate :: BitReader < INT_EVENT0_IMASK_OVIFG_A > ; # [doc = "Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_OVIFG_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_OVIFG_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_OVIFG_SET = 1 , } impl From < INT_EVENT0_IMASK_OVIFG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_OVIFG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_OVIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_OVIFG_A { match self . bits { false => INT_EVENT0_IMASK_OVIFG_A :: INT_EVENT0_IMASK_OVIFG_CLR , true => INT_EVENT0_IMASK_OVIFG_A :: INT_EVENT0_IMASK_OVIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_ovifg_clr (& self) -> bool { * self == INT_EVENT0_IMASK_OVIFG_A :: INT_EVENT0_IMASK_OVIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_ovifg_set (& self) -> bool { * self == INT_EVENT0_IMASK_OVIFG_A :: INT_EVENT0_IMASK_OVIFG_SET } } # [doc = "Field `INT_EVENT0_IMASK_OVIFG` writer - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_OVIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_OVIFG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_OVIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_ovifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_OVIFG_A :: INT_EVENT0_IMASK_OVIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_ovifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_OVIFG_A :: INT_EVENT0_IMASK_OVIFG_SET) } } # [doc = "Field `INT_EVENT0_IMASK_TOVIFG` reader - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_TOVIFG_R = crate :: BitReader < INT_EVENT0_IMASK_TOVIFG_A > ; # [doc = "Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_TOVIFG_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_TOVIFG_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_TOVIFG_SET = 1 , } impl From < INT_EVENT0_IMASK_TOVIFG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_TOVIFG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_TOVIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_TOVIFG_A { match self . bits { false => INT_EVENT0_IMASK_TOVIFG_A :: INT_EVENT0_IMASK_TOVIFG_CLR , true => INT_EVENT0_IMASK_TOVIFG_A :: INT_EVENT0_IMASK_TOVIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_tovifg_clr (& self) -> bool { * self == INT_EVENT0_IMASK_TOVIFG_A :: INT_EVENT0_IMASK_TOVIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_tovifg_set (& self) -> bool { * self == INT_EVENT0_IMASK_TOVIFG_A :: INT_EVENT0_IMASK_TOVIFG_SET } } # [doc = "Field `INT_EVENT0_IMASK_TOVIFG` writer - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_TOVIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_TOVIFG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_TOVIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_tovifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_TOVIFG_A :: INT_EVENT0_IMASK_TOVIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_tovifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_TOVIFG_A :: INT_EVENT0_IMASK_TOVIFG_SET) } } # [doc = "Field `INT_EVENT0_IMASK_HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_HIGHIFG_R = crate :: BitReader < INT_EVENT0_IMASK_HIGHIFG_A > ; # [doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_HIGHIFG_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_HIGHIFG_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_HIGHIFG_SET = 1 , } impl From < INT_EVENT0_IMASK_HIGHIFG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_HIGHIFG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_HIGHIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_HIGHIFG_A { match self . bits { false => INT_EVENT0_IMASK_HIGHIFG_A :: INT_EVENT0_IMASK_HIGHIFG_CLR , true => INT_EVENT0_IMASK_HIGHIFG_A :: INT_EVENT0_IMASK_HIGHIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_highifg_clr (& self) -> bool { * self == INT_EVENT0_IMASK_HIGHIFG_A :: INT_EVENT0_IMASK_HIGHIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_highifg_set (& self) -> bool { * self == INT_EVENT0_IMASK_HIGHIFG_A :: INT_EVENT0_IMASK_HIGHIFG_SET } } # [doc = "Field `INT_EVENT0_IMASK_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_HIGHIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_HIGHIFG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_HIGHIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_highifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_HIGHIFG_A :: INT_EVENT0_IMASK_HIGHIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_highifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_HIGHIFG_A :: INT_EVENT0_IMASK_HIGHIFG_SET) } } # [doc = "Field `INT_EVENT0_IMASK_LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_LOWIFG_R = crate :: BitReader < INT_EVENT0_IMASK_LOWIFG_A > ; # [doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_LOWIFG_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_LOWIFG_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_LOWIFG_SET = 1 , } impl From < INT_EVENT0_IMASK_LOWIFG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_LOWIFG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_LOWIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_LOWIFG_A { match self . bits { false => INT_EVENT0_IMASK_LOWIFG_A :: INT_EVENT0_IMASK_LOWIFG_CLR , true => INT_EVENT0_IMASK_LOWIFG_A :: INT_EVENT0_IMASK_LOWIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_lowifg_clr (& self) -> bool { * self == INT_EVENT0_IMASK_LOWIFG_A :: INT_EVENT0_IMASK_LOWIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_lowifg_set (& self) -> bool { * self == INT_EVENT0_IMASK_LOWIFG_A :: INT_EVENT0_IMASK_LOWIFG_SET } } # [doc = "Field `INT_EVENT0_IMASK_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_LOWIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_LOWIFG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_LOWIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_lowifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_LOWIFG_A :: INT_EVENT0_IMASK_LOWIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_lowifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_LOWIFG_A :: INT_EVENT0_IMASK_LOWIFG_SET) } } # [doc = "Field `INT_EVENT0_IMASK_INIFG` reader - Mask INIFG in MIS_EX register."] pub type INT_EVENT0_IMASK_INIFG_R = crate :: BitReader < INT_EVENT0_IMASK_INIFG_A > ; # [doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_INIFG_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_INIFG_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_INIFG_SET = 1 , } impl From < INT_EVENT0_IMASK_INIFG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_INIFG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_INIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_INIFG_A { match self . bits { false => INT_EVENT0_IMASK_INIFG_A :: INT_EVENT0_IMASK_INIFG_CLR , true => INT_EVENT0_IMASK_INIFG_A :: INT_EVENT0_IMASK_INIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_inifg_clr (& self) -> bool { * self == INT_EVENT0_IMASK_INIFG_A :: INT_EVENT0_IMASK_INIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_inifg_set (& self) -> bool { * self == INT_EVENT0_IMASK_INIFG_A :: INT_EVENT0_IMASK_INIFG_SET } } # [doc = "Field `INT_EVENT0_IMASK_INIFG` writer - Mask INIFG in MIS_EX register."] pub type INT_EVENT0_IMASK_INIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_INIFG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_INIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_inifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_INIFG_A :: INT_EVENT0_IMASK_INIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_inifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_INIFG_A :: INT_EVENT0_IMASK_INIFG_SET) } } # [doc = "Field `INT_EVENT0_IMASK_DMADONE` reader - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_DMADONE_R = crate :: BitReader < INT_EVENT0_IMASK_DMADONE_A > ; # [doc = "Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_DMADONE_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_DMADONE_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_DMADONE_SET = 1 , } impl From < INT_EVENT0_IMASK_DMADONE_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_DMADONE_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_DMADONE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_DMADONE_A { match self . bits { false => INT_EVENT0_IMASK_DMADONE_A :: INT_EVENT0_IMASK_DMADONE_CLR , true => INT_EVENT0_IMASK_DMADONE_A :: INT_EVENT0_IMASK_DMADONE_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_dmadone_clr (& self) -> bool { * self == INT_EVENT0_IMASK_DMADONE_A :: INT_EVENT0_IMASK_DMADONE_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_dmadone_set (& self) -> bool { * self == INT_EVENT0_IMASK_DMADONE_A :: INT_EVENT0_IMASK_DMADONE_SET } } # [doc = "Field `INT_EVENT0_IMASK_DMADONE` writer - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT0_IMASK_DMADONE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_DMADONE_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_DMADONE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_dmadone_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_DMADONE_A :: INT_EVENT0_IMASK_DMADONE_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_dmadone_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_DMADONE_A :: INT_EVENT0_IMASK_DMADONE_SET) } } # [doc = "Field `INT_EVENT0_IMASK_UVIFG` reader - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."] pub type INT_EVENT0_IMASK_UVIFG_R = crate :: BitReader < INT_EVENT0_IMASK_UVIFG_A > ; # [doc = "Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_UVIFG_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_UVIFG_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_UVIFG_SET = 1 , } impl From < INT_EVENT0_IMASK_UVIFG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_UVIFG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_UVIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_UVIFG_A { match self . bits { false => INT_EVENT0_IMASK_UVIFG_A :: INT_EVENT0_IMASK_UVIFG_CLR , true => INT_EVENT0_IMASK_UVIFG_A :: INT_EVENT0_IMASK_UVIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_uvifg_clr (& self) -> bool { * self == INT_EVENT0_IMASK_UVIFG_A :: INT_EVENT0_IMASK_UVIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_uvifg_set (& self) -> bool { * self == INT_EVENT0_IMASK_UVIFG_A :: INT_EVENT0_IMASK_UVIFG_SET } } # [doc = "Field `INT_EVENT0_IMASK_UVIFG` writer - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."] pub type INT_EVENT0_IMASK_UVIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_UVIFG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_UVIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_uvifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_UVIFG_A :: INT_EVENT0_IMASK_UVIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_uvifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_UVIFG_A :: INT_EVENT0_IMASK_UVIFG_SET) } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG0_R = crate :: BitReader < INT_EVENT0_IMASK_MEMRESIFG0_A > ; # [doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_MEMRESIFG0_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_MEMRESIFG0_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_MEMRESIFG0_SET = 1 , } impl From < INT_EVENT0_IMASK_MEMRESIFG0_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_MEMRESIFG0_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_MEMRESIFG0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_MEMRESIFG0_A { match self . bits { false => INT_EVENT0_IMASK_MEMRESIFG0_A :: INT_EVENT0_IMASK_MEMRESIFG0_CLR , true => INT_EVENT0_IMASK_MEMRESIFG0_A :: INT_EVENT0_IMASK_MEMRESIFG0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_memresifg0_clr (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG0_A :: INT_EVENT0_IMASK_MEMRESIFG0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_memresifg0_set (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG0_A :: INT_EVENT0_IMASK_MEMRESIFG0_SET } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_MEMRESIFG0_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_MEMRESIFG0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_memresifg0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG0_A :: INT_EVENT0_IMASK_MEMRESIFG0_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_memresifg0_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG0_A :: INT_EVENT0_IMASK_MEMRESIFG0_SET) } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG1_R = crate :: BitReader < INT_EVENT0_IMASK_MEMRESIFG1_A > ; # [doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_MEMRESIFG1_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_MEMRESIFG1_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_MEMRESIFG1_SET = 1 , } impl From < INT_EVENT0_IMASK_MEMRESIFG1_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_MEMRESIFG1_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_MEMRESIFG1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_MEMRESIFG1_A { match self . bits { false => INT_EVENT0_IMASK_MEMRESIFG1_A :: INT_EVENT0_IMASK_MEMRESIFG1_CLR , true => INT_EVENT0_IMASK_MEMRESIFG1_A :: INT_EVENT0_IMASK_MEMRESIFG1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_memresifg1_clr (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG1_A :: INT_EVENT0_IMASK_MEMRESIFG1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_memresifg1_set (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG1_A :: INT_EVENT0_IMASK_MEMRESIFG1_SET } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG1_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_MEMRESIFG1_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_MEMRESIFG1_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_memresifg1_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG1_A :: INT_EVENT0_IMASK_MEMRESIFG1_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_memresifg1_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG1_A :: INT_EVENT0_IMASK_MEMRESIFG1_SET) } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG2_R = crate :: BitReader < INT_EVENT0_IMASK_MEMRESIFG2_A > ; # [doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_MEMRESIFG2_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_MEMRESIFG2_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_MEMRESIFG2_SET = 1 , } impl From < INT_EVENT0_IMASK_MEMRESIFG2_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_MEMRESIFG2_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_MEMRESIFG2_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_MEMRESIFG2_A { match self . bits { false => INT_EVENT0_IMASK_MEMRESIFG2_A :: INT_EVENT0_IMASK_MEMRESIFG2_CLR , true => INT_EVENT0_IMASK_MEMRESIFG2_A :: INT_EVENT0_IMASK_MEMRESIFG2_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_memresifg2_clr (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG2_A :: INT_EVENT0_IMASK_MEMRESIFG2_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_memresifg2_set (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG2_A :: INT_EVENT0_IMASK_MEMRESIFG2_SET } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG2_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_MEMRESIFG2_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_MEMRESIFG2_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_memresifg2_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG2_A :: INT_EVENT0_IMASK_MEMRESIFG2_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_memresifg2_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG2_A :: INT_EVENT0_IMASK_MEMRESIFG2_SET) } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG3_R = crate :: BitReader < INT_EVENT0_IMASK_MEMRESIFG3_A > ; # [doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_IMASK_MEMRESIFG3_A { # [doc = "0: CLR"] INT_EVENT0_IMASK_MEMRESIFG3_CLR = 0 , # [doc = "1: SET"] INT_EVENT0_IMASK_MEMRESIFG3_SET = 1 , } impl From < INT_EVENT0_IMASK_MEMRESIFG3_A > for bool { # [inline (always)] fn from (variant : INT_EVENT0_IMASK_MEMRESIFG3_A) -> Self { variant as u8 != 0 } } impl INT_EVENT0_IMASK_MEMRESIFG3_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT0_IMASK_MEMRESIFG3_A { match self . bits { false => INT_EVENT0_IMASK_MEMRESIFG3_A :: INT_EVENT0_IMASK_MEMRESIFG3_CLR , true => INT_EVENT0_IMASK_MEMRESIFG3_A :: INT_EVENT0_IMASK_MEMRESIFG3_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event0_imask_memresifg3_clr (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG3_A :: INT_EVENT0_IMASK_MEMRESIFG3_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event0_imask_memresifg3_set (& self) -> bool { * self == INT_EVENT0_IMASK_MEMRESIFG3_A :: INT_EVENT0_IMASK_MEMRESIFG3_SET } } # [doc = "Field `INT_EVENT0_IMASK_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT0_IMASK_MEMRESIFG3_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_IMASK_MEMRESIFG3_A > ; impl < 'a , REG , const O : u8 > INT_EVENT0_IMASK_MEMRESIFG3_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event0_imask_memresifg3_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG3_A :: INT_EVENT0_IMASK_MEMRESIFG3_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event0_imask_memresifg3_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_IMASK_MEMRESIFG3_A :: INT_EVENT0_IMASK_MEMRESIFG3_SET) } } impl R { # [doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] pub fn int_event0_imask_ovifg (& self) -> INT_EVENT0_IMASK_OVIFG_R { INT_EVENT0_IMASK_OVIFG_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] pub fn int_event0_imask_tovifg (& self) -> INT_EVENT0_IMASK_TOVIFG_R { INT_EVENT0_IMASK_TOVIFG_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] pub fn int_event0_imask_highifg (& self) -> INT_EVENT0_IMASK_HIGHIFG_R { INT_EVENT0_IMASK_HIGHIFG_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] pub fn int_event0_imask_lowifg (& self) -> INT_EVENT0_IMASK_LOWIFG_R { INT_EVENT0_IMASK_LOWIFG_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - Mask INIFG in MIS_EX register."] # [inline (always)] pub fn int_event0_imask_inifg (& self) -> INT_EVENT0_IMASK_INIFG_R { INT_EVENT0_IMASK_INIFG_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] pub fn int_event0_imask_dmadone (& self) -> INT_EVENT0_IMASK_DMADONE_R { INT_EVENT0_IMASK_DMADONE_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."] # [inline (always)] pub fn int_event0_imask_uvifg (& self) -> INT_EVENT0_IMASK_UVIFG_R { INT_EVENT0_IMASK_UVIFG_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event0_imask_memresifg0 (& self) -> INT_EVENT0_IMASK_MEMRESIFG0_R { INT_EVENT0_IMASK_MEMRESIFG0_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event0_imask_memresifg1 (& self) -> INT_EVENT0_IMASK_MEMRESIFG1_R { INT_EVENT0_IMASK_MEMRESIFG1_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event0_imask_memresifg2 (& self) -> INT_EVENT0_IMASK_MEMRESIFG2_R { INT_EVENT0_IMASK_MEMRESIFG2_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event0_imask_memresifg3 (& self) -> INT_EVENT0_IMASK_MEMRESIFG3_R { INT_EVENT0_IMASK_MEMRESIFG3_R :: new (((self . bits >> 11) & 1) != 0) } } impl W { # [doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] # [must_use] pub fn int_event0_imask_ovifg (& mut self) -> INT_EVENT0_IMASK_OVIFG_W < INT_EVENT0_IMASK_SPEC , 0 > { INT_EVENT0_IMASK_OVIFG_W :: new (self) } # [doc = "Bit 1 - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] # [must_use] pub fn int_event0_imask_tovifg (& mut self) -> INT_EVENT0_IMASK_TOVIFG_W < INT_EVENT0_IMASK_SPEC , 1 > { INT_EVENT0_IMASK_TOVIFG_W :: new (self) } # [doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] # [must_use] pub fn int_event0_imask_highifg (& mut self) -> INT_EVENT0_IMASK_HIGHIFG_W < INT_EVENT0_IMASK_SPEC , 2 > { INT_EVENT0_IMASK_HIGHIFG_W :: new (self) } # [doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] # [must_use] pub fn int_event0_imask_lowifg (& mut self) -> INT_EVENT0_IMASK_LOWIFG_W < INT_EVENT0_IMASK_SPEC , 3 > { INT_EVENT0_IMASK_LOWIFG_W :: new (self) } # [doc = "Bit 4 - Mask INIFG in MIS_EX register."] # [inline (always)] # [must_use] pub fn int_event0_imask_inifg (& mut self) -> INT_EVENT0_IMASK_INIFG_W < INT_EVENT0_IMASK_SPEC , 4 > { INT_EVENT0_IMASK_INIFG_W :: new (self) } # [doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] # [must_use] pub fn int_event0_imask_dmadone (& mut self) -> INT_EVENT0_IMASK_DMADONE_W < INT_EVENT0_IMASK_SPEC , 5 > { INT_EVENT0_IMASK_DMADONE_W :: new (self) } # [doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."] # [inline (always)] # [must_use] pub fn int_event0_imask_uvifg (& mut self) -> INT_EVENT0_IMASK_UVIFG_W < INT_EVENT0_IMASK_SPEC , 6 > { INT_EVENT0_IMASK_UVIFG_W :: new (self) } # [doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event0_imask_memresifg0 (& mut self) -> INT_EVENT0_IMASK_MEMRESIFG0_W < INT_EVENT0_IMASK_SPEC , 8 > { INT_EVENT0_IMASK_MEMRESIFG0_W :: new (self) } # [doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event0_imask_memresifg1 (& mut self) -> INT_EVENT0_IMASK_MEMRESIFG1_W < INT_EVENT0_IMASK_SPEC , 9 > { INT_EVENT0_IMASK_MEMRESIFG1_W :: new (self) } # [doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event0_imask_memresifg2 (& mut self) -> INT_EVENT0_IMASK_MEMRESIFG2_W < INT_EVENT0_IMASK_SPEC , 10 > { INT_EVENT0_IMASK_MEMRESIFG2_W :: new (self) } # [doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event0_imask_memresifg3 (& mut self) -> INT_EVENT0_IMASK_MEMRESIFG3_W < INT_EVENT0_IMASK_SPEC , 11 > { INT_EVENT0_IMASK_MEMRESIFG3_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT0_IMASK_SPEC ; impl crate :: RegisterSpec for INT_EVENT0_IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"] impl crate :: Readable for INT_EVENT0_IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"] impl crate :: Writable for INT_EVENT0_IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"] impl crate :: Resettable for INT_EVENT0_IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }