#define SYSCALL_cx_math_multm_ID_IN 0x60004445UL
#define SYSCALL_cx_math_multm_ID_OUT 0x900044f3UL

unsigned int SVC_Call(unsigned int syscall_id, unsigned int * parameters) __attribute__ ((naked));

unsigned int SVC_Call(unsigned int syscall_id, unsigned int * parameters) {
    // delegate svc
    asm volatile("svc #1":::"r0","r1");
    // directly return R0 value
    asm volatile("bx  lr");
}

void cx_math_multm ( unsigned char * r, const unsigned char * a, const unsigned char * b, const unsigned char * m, unsigned int len ) 
{
  unsigned int ret;
  unsigned int retid;
  unsigned int parameters [0+5];
  parameters[0] = (unsigned int)r;
  parameters[1] = (unsigned int)a;
  parameters[2] = (unsigned int)b;
  parameters[3] = (unsigned int)m;
  parameters[4] = (unsigned int)len;
  retid = SVC_Call(SYSCALL_cx_math_multm_ID_IN, parameters);
  asm volatile("str r1, %0":"=m"(ret)::"r1");
/*  if (retid != SYSCALL_cx_math_multm_ID_OUT) {
    THROW(EXCEPTION_SECURITY);
  }*/
}
