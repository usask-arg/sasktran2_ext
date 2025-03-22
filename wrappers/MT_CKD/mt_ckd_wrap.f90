module mtckd_wrapper_mod
  use iso_c_binding
  Use lblparams
  USE phys_consts
  USE planet_consts
  IMPLICIT REAL*8           (V)

contains

  subroutine set_inputs(pave_input, tave_input, vmr_h2o, vmr_co2, vmr_o3, path_length) bind(C)
    real(c_double), value :: pave_input, tave_input, vmr_h2o, vmr_co2, vmr_o3, path_length

    COMMON /ABSORB/ V1ABS,V2ABS,DVABS,NPTABS,ABSRB(5050)              ! 500060

    COMMON /CVRCNT/ HNAMCNT,HVRCNT

    COMMON /FILHDR/ XID(10),SECANT,PAVE,TAVE,HMOLID(60),XALTZ(4),  &    ! F00130
                    WK(60),PZL,PZU,TZL,TZU,WBROAD,DV ,V1 ,V2 ,TBOUND, &  ! F00140
                    EMISIV,FSCDID(17),NMOL,LAYER ,YI1,YID(10),LSTWDF   ! F00150

    Common /share/ HOLN2

    COMMON /CONSTS/ CPDAIR,SECDY  
    COMMON /LAMCHN/ ONEPL,ONEMI,EXPMIN,ARGMIN                          ! A03020
    COMMON /XCONT/  V1C,V2C,DVC,NPTC,C(6000) 

    COMMON /cnth2o/ V1h,V2h,DVh,NPTh,Ch(5050),csh2o(5050),cfh2o(5050)
    COMMON /IFIL/ IRD,IPRcnt,IPU,NOPR,NFHDRF,NPHDRF,NFHDRL,NPHDRL, &
                NLNGTH,KFILE,KPANEL,LINFIL,NFILE,IAFIL,IEXFIL,   &    ! F00180
                NLTEFL,LNFIL4,LNGTH4                                 ! F00190

    common /cntscl/ XSELF,XFRGN,XCO2C,XO3CN,XO2CN,XN2CN,XRAYL

    common /CDERIV/ icflg,iuf,v1absc,v2absc,dvabsc,nptabsc, &
    dqh2oC(ipts),dTh2oC(ipts),dUh2o, &
    dqco2C(ipts),dTco2C(ipts), &
    dqo3C(ipts),dTo3C(ipts), &
    dqo2C(ipts),dTo2C(ipts), &
    dqn2C(ipts),dTn2C(ipts)

    real ctmp(ipts2),cself(ipts),cforeign(ipts),ch2o(ipts2)
    real ctmp2(ipts2),ctmp3(ipts2),ctmp4(ipts),ctmp5(ipts)

    dimension xcnt(7)
    !
          equivalence (xself,xcnt(1))
    !
          CHARACTER*18 HNAMCNT,HVRCNT
    !                                                                         F00100
          CHARACTER*8      XID,       HMOLID,      YID 
          REAL*8               SECANT,       XALTZ
    !
          character*8 holn2

    icflg = -999
    do 1, i=1,7
        xcnt(i)=1.
1    continue

    do 2, i=1,5050
        absrb(i)=0.
2    continue

    do 3, i=1,60
        wk(i)=0.
3    continue

    ird = 55
    ipr = 66
    ipu = 7

    pave = pave_input
    tave = tave_input

    xlength = path_length

    VMRH2O = vmr_h2o

    print 911, pave,tave,xlength,vmrh2o
    911  format(1x,f13.6,f17.4,f18.4,f12.8)

    WTOT = ALOSMT*(PAVE/1013.)*(273./TAVE)* xlength
!
    W_dry = WTOT * (1.-VMRH2O)
! argon:
    WA     = 0.009     * W_dry
! nitrogen:
    WN2    = 0.78      * W_dry
! oxygen:
    WK(7)  = 0.21      * W_dry

! carbon dioxide:
    WK(2)  = vmr_co2  * W_dry

!      WK(2) = 0.
! ozone:
!      WK(3) = 3.75E-6 * W_dry
    WK(3) = 0.
! water vapor:
    if (abs(vmrh2o-1.) .lt. 1.e-05) then
        wk(1) = wtot
    else
        WK(1) = VMRH2O * W_dry
    endif
!
    WBROAD=WN2+WA
!
    NMOL = 7
!
    V1ABS =     0.
    V2ABS = 19900. 

    DVABS =    10.

    NPTABS =  1. + (v2abs-v1abs)/dvabs

    do 85 i=1,nptabs
        absrb(i) =0.
85   continue

    v1 = v1abs
    v2 = v2abs

  end subroutine

  subroutine run_mtckd() bind(C)
    integer(c_int) :: jrad
    jrad = 0
    call contnm(jrad)
  end subroutine

  function get_absrb(i) result(val) bind(C)
    integer(c_int), value :: i
    real(c_double) :: val

    COMMON /ABSORB/ V1ABS,V2ABS,DVABS,NPTABS,ABSRB(5050)              ! 500060
    COMMON /cnth2o/ V1h,V2h,DVh,NPTh,Ch(5050),csh2o(5050),cfh2o(5050)
    COMMON /FILHDR/ XID(10),SECANT,PAVE,TAVE,HMOLID(60),XALTZ(4),  &    ! F00130
                    WK(60),PZL,PZU,TZL,TZU,WBROAD,DV ,V1 ,V2 ,TBOUND, &  ! F00140
                    EMISIV,FSCDID(17),NMOL,LAYER ,YI1,YID(10),LSTWDF   ! F00150

    xkt=tave/radcn2
    vi=v1h+float(i-1)*dvh
    val = absrb(i) * radfn(vi, xkt)

  end function

end module