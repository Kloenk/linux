/* SPDX-License-Identifier: GPL-2.0 */

#include <linux/cdev.h>
#include <linux/fs.h>
#include <linux/netdevice.h>
#include <linux/ethtool.h>
#include <linux/etherdevice.h>
#include <linux/netdev_features.h>
#include <linux/rtnetlink.h>
#include <net/rtnetlink.h>
#include <linux/module.h>
#include <linux/random.h>
#include <linux/slab.h>
#include <linux/sysctl.h>
#include <linux/uaccess.h>
#include <linux/uio.h>
#include <linux/version.h>
#include <linux/miscdevice.h>
#include <linux/poll.h>
#include <linux/mm.h>
#include <uapi/linux/android/binder.h>
#include <crypto/blake2s.h>

// `bindgen` gets confused at certain things
const gfp_t BINDINGS_GFP_KERNEL = GFP_KERNEL;
const gfp_t BINDINGS___GFP_ZERO = __GFP_ZERO;

const int BINDINGS_NLA_HDRLEN = NLA_HDRLEN;
