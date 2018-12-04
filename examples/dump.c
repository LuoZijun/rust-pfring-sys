#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <sys/types.h>
#include <errno.h>
#include <time.h>

#include "../include/pfring.h"
#include "../include/pfring_ft.h"
#include "../include/pfring_zc.h"
#include "../include/pfring_utils.h"
#include "../include/pfring_mod_stack.h"
#include "../include/pfring_mod_sysdig.h"
#include "../include/linux/pf_ring.h"

// 
// cc examples/dump.c -Wall -g -lpfring -lpcap -o dump
// 
// cc dump.o -L /usr/local/lib -lpfring -lpcap -o dump
// 
// apt install bison flex
// make libring
// make pcap
// make build_examples
// 

void dummyProcessPacket(const struct pfring_pkthdr *h,
                        const u_char *p,
                        const u_char *user_bytes) {

}

bool is_root() {
    if ( getuid() == 0 ) {
        return true;
    } else {
        return false;
    }
}

int main(int argc, char const *argv[]) {
    if ( !is_root() ) {
        printf("[ERROR] PermissionDenied\n");
        exit(1);
    }

    pfring  *pd;
    pd = pfring_open("enp3s0", 1500, PF_RING_TIMESTAMP);

    if( pd == NULL ) {
        fprintf(stderr, "pfring_open error [%s] (pf_ring not loaded or interface `enp3s0` is down ?)\n",
            strerror(errno) );
        return(-1);
    }

    printf("pfring initialized: %d\n", pd->initialized ) ;
    printf("pfring enabled: %d\n", pd->enabled ) ;

    u_int32_t version;
    pfring_set_application_name(pd, "pfrecv");
    pfring_version(pd, &version);

    printf("Using PF_RING v.%d.%d.%d\n",
         (version & 0xFFFF0000) >> 16,
         (version & 0x0000FF00) >> 8,
         version & 0x000000FF);

    if ( pfring_enable_ring(pd) != 0 ) {
        fprintf(stderr, "Unable to enable ring :-(\n");
        pfring_close(pd);
        return(-1);
    }

    u_int  len = 1500;
    u_int8_t wait_for_packet = 1;
    struct pfring_pkthdr hdr;

    u_char buffer[len];
    u_char *buffer_p = buffer;

    memset(&hdr, 0, sizeof(hdr));


    while (pfring_recv(pd, &buffer_p, len, &hdr, wait_for_packet) == 1) {
        for (int i=0; i<hdr.len; i++) {
            printf("%d, ", buffer[i]);
        }
        printf("\n");
    }    

    // pfring_loop(pd, dummyProcessPacket, (u_char*)NULL, wait_for_packet);
    
    return 0;
}
