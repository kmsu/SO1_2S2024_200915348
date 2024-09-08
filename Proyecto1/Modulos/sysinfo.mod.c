#include <linux/module.h>
#define INCLUDE_VERMAGIC
#include <linux/build-salt.h>
#include <linux/elfnote-lto.h>
#include <linux/export-internal.h>
#include <linux/vermagic.h>
#include <linux/compiler.h>

#ifdef CONFIG_UNWINDER_ORC
#include <asm/orc_header.h>
ORC_HEADER;
#endif

BUILD_SALT;
BUILD_LTO_INFO;

MODULE_INFO(vermagic, VERMAGIC_STRING);
MODULE_INFO(name, KBUILD_MODNAME);

__visible struct module __this_module
__section(".gnu.linkonce.this_module") = {
	.name = KBUILD_MODNAME,
	.init = init_module,
#ifdef CONFIG_MODULE_UNLOAD
	.exit = cleanup_module,
#endif
	.arch = MODULE_ARCH_INIT,
};

#ifdef CONFIG_RETPOLINE
MODULE_INFO(retpoline, "Y");
#endif



static const struct modversion_info ____versions[]
__used __section("__versions") = {
	{ 0x3edadd3b, "single_open" },
	{ 0x15ba50a6, "jiffies" },
	{ 0x40c7247c, "si_meminfo" },
	{ 0x6d26b3a7, "seq_printf" },
	{ 0x441c5e2e, "init_task" },
	{ 0xe2d5255a, "strcmp" },
	{ 0x4c03a563, "random_kmalloc_seed" },
	{ 0xf2999cd4, "kmalloc_caches" },
	{ 0x4524dc3a, "kmalloc_trace" },
	{ 0xd0df9f72, "get_task_mm" },
	{ 0x668b19a1, "down_read" },
	{ 0x53b954a2, "up_read" },
	{ 0x91cb5be4, "access_process_vm" },
	{ 0x5c1531ec, "mmput" },
	{ 0x37a0cba, "kfree" },
	{ 0xf0fdf6cb, "__stack_chk_fail" },
	{ 0x2394a322, "remove_proc_entry" },
	{ 0x2fa57770, "seq_read" },
	{ 0xbdfb6dbb, "__fentry__" },
	{ 0x6e2d7b65, "proc_create" },
	{ 0x122c3a7e, "_printk" },
	{ 0x5b8239ca, "__x86_return_thunk" },
	{ 0x708cd699, "module_layout" },
};

MODULE_INFO(depends, "");


MODULE_INFO(srcversion, "D1E094215CD5A363F933EC3");
