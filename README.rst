rust pfring-sys
====================

:Date: 2018/12/04

.. contents::


Build
----------

安装预编译的 PF_RING 内核模块以及用户态的 API:

.. code:: bash
    
    # Debian
    wget http://apt.ntop.org/stretch/all/apt-ntop.deb
    dpkg -i apt-ntop.deb 
    apt install pfring pfring-dkms pfring-drivers-zc-dkms


从源码开始编译 PF_RING 内核模块 以及用户态 API:

.. code:: bash
    
    # Debian
    sudo apt install bison flex

    git clone https://github.com/ntop/PF_RING.git

    cd  PF_RING/driver
    make

    cd  PF_RING/kernel
    make

    cd PF_RING/userland
    make libpfring pcap build_examples


构建对 Rust 的绑定:

.. code:: bash
    
    cargo build



使用样例
-----------------

Rust 语言样例:

.. code:: bash
    
    cargo build --example dump
    sudo ./target/debug/dump


C 语言样例:

.. code:: bash

    cc examples/dump.c -Wall -g -lpfring -lpcap -o dump
    sudo ./dump



支持的网卡驱动
-----------------

*   https://www.kernel.org/doc/Documentation/networking/e1000e.txt


参考
--------


*   `PF RING API <http://www.ntop.org/guides/pf_ring_api>`_
*   `PF RING Guide <http://www.ntop.org/guides/pf_ring>`_
*   `PF RING Get Started <https://www.ntop.org/get-started/download>`_
*   `PF RING installation guide <https://www.ntop.org/pf_ring/installation-guide-for-pf_ring/>`_
*   `PF RING Package <http://packages.ntop.org/>`_
*   `Github ntop/PF_RING <https://github.com/ntop/PF_RING>`_

