// SPDX-License-Identifier: GPL-2.0

#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/uaccess.h>
#include <linux/sched/signal.h>
#include <linux/gfp.h>
#include <linux/highmem.h>
#include <linux/uio.h>
#include <linux/netdevice.h>
#include <linux/etherdevice.h>
#include <linux/rtnetlink.h>

void rust_helper_BUG(void)
{
	BUG();
}

unsigned long rust_helper_copy_from_user(void *to, const void __user *from, unsigned long n)
{
	return copy_from_user(to, from, n);
}

unsigned long rust_helper_copy_to_user(void __user *to, const void *from, unsigned long n)
{
	return copy_to_user(to, from, n);
}

unsigned long rust_helper_clear_user(void __user *to, unsigned long n)
{
	return clear_user(to, n);
}

void rust_helper_spin_lock_init(spinlock_t *lock, const char *name,
				struct lock_class_key *key)
{
#ifdef CONFIG_DEBUG_SPINLOCK
	__spin_lock_init(lock, name, key);
#else
	spin_lock_init(lock);
#endif
}
EXPORT_SYMBOL_GPL(rust_helper_spin_lock_init);

void rust_helper_spin_lock(spinlock_t *lock)
{
	spin_lock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_lock);

void rust_helper_spin_unlock(spinlock_t *lock)
{
	spin_unlock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_unlock);

void rust_helper_init_wait(struct wait_queue_entry *wq_entry)
{
	init_wait(wq_entry);
}
EXPORT_SYMBOL_GPL(rust_helper_init_wait);

int rust_helper_current_pid(void)
{
	return current->pid;
}
EXPORT_SYMBOL_GPL(rust_helper_current_pid);

int rust_helper_signal_pending(void)
{
	return signal_pending(current);
}
EXPORT_SYMBOL_GPL(rust_helper_signal_pending);

struct page *rust_helper_alloc_pages(gfp_t gfp_mask, unsigned int order)
{
	return alloc_pages(gfp_mask, order);
}
EXPORT_SYMBOL_GPL(rust_helper_alloc_pages);

void *rust_helper_kmap(struct page *page)
{
	return kmap(page);
}
EXPORT_SYMBOL_GPL(rust_helper_kmap);

void rust_helper_kunmap(struct page *page)
{
	return kunmap(page);
}
EXPORT_SYMBOL_GPL(rust_helper_kunmap);

int rust_helper_cond_resched(void)
{
	return cond_resched();
}
EXPORT_SYMBOL_GPL(rust_helper_cond_resched);

size_t rust_helper_copy_from_iter(void *addr, size_t bytes, struct iov_iter *i)
{
	return copy_from_iter(addr, bytes, i);
}
EXPORT_SYMBOL_GPL(rust_helper_copy_from_iter);

size_t rust_helper_copy_to_iter(const void *addr, size_t bytes, struct iov_iter *i)
{
	return copy_to_iter(addr, bytes, i);
}
EXPORT_SYMBOL_GPL(rust_helper_copy_to_iter);

void *rust_helper_netdev_priv(struct net_device *dev)
{
	return netdev_priv(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_netdev_priv);

void rust_helper_eth_hw_addr_random(struct net_device *dev)
{
	eth_hw_addr_random(dev);
}
EXPORT_SYMBOL_GPL(rust_helper_eth_hw_addr_random);

int rust_helper_net_device_set_new_lstats(struct net_device *dev)
{
	dev->lstats = netdev_alloc_pcpu_stats(struct pcpu_lstats);
	if (!dev->lstats)
		return -ENOMEM;

	return 0;
}
EXPORT_SYMBOL_GPL(rust_helper_net_device_set_new_lstats);

void rust_helper_dev_lstats_add(struct net_device *dev, unsigned int len)
{
	dev_lstats_add(dev, len);
}
EXPORT_SYMBOL_GPL(rust_helper_dev_lstats_add);

// See https://github.com/rust-lang/rust-bindgen/issues/1671
#if !defined(CONFIG_ARM)
static_assert(__builtin_types_compatible_p(size_t, uintptr_t),
	"size_t must match uintptr_t, what architecture is this??");
#endif
