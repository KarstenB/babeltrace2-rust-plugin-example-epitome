// Copyright 2023 - 2023, Karsten Becker and the rust babeltrace2-plugin contributors
// SPDX-License-Identifier: GPL-2.0-or-later
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <babeltrace2/babeltrace.h>
#include "epitome.h"

/* Mandatory */
BT_PLUGIN_MODULE();
 
/* Define the `epitome` plugin */
BT_PLUGIN(epitome);
 
/* Define the `output` sink component class */
BT_PLUGIN_SINK_COMPONENT_CLASS(output, epitome_out_consume);
 
/* Set some of the `output` sink component class's optional methods */
BT_PLUGIN_SINK_COMPONENT_CLASS_INITIALIZE_METHOD(output,
    epitome_out_initialize);
BT_PLUGIN_SINK_COMPONENT_CLASS_FINALIZE_METHOD(output, epitome_out_finalize);
BT_PLUGIN_SINK_COMPONENT_CLASS_GRAPH_IS_CONFIGURED_METHOD(output,
    epitome_out_graph_is_configured);