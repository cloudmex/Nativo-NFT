#!/usr/bin/env awk -f

BEGIN {
  sym_prefix = "_"
  split("\
        __rjem_aligned_alloc \
        __rjem_calloc \
        __rjem_dallocx \
        __rjem_free \
        __rjem_mallctl \
        __rjem_mallctlbymib \
        __rjem_mallctlnametomib \
        __rjem_malloc \
        __rjem_malloc_conf \
        __rjem_malloc_message \
        __rjem_malloc_stats_print \
        __rjem_malloc_usable_size \
        __rjem_mallocx \
        __rjem_nallocx \
        __rjem_posix_memalign \
        __rjem_rallocx \
        __rjem_realloc \
        __rjem_sallocx \
        __rjem_sdallocx \
        __rjem_xallocx \
        __rjem_valloc \
        _pthread_create \
        ", exported_symbol_names)
  # Store exported symbol names as keys in exported_symbols.
  for (i in exported_symbol_names) {
    exported_symbols[exported_symbol_names[i]] = 1
  }
}

# Process 'nm -a <c_source.o>' output.
#
# Handle lines like:
#   0000000000000008 D opt_junk
#   0000000000007574 T malloc_initialized
(NF == 3 && $2 ~ /^[ABCDGRSTVW]$/ && !($3 in exported_symbols) && $3 ~ /^[A-Za-z0-9_]+$/) {
  print substr($3, 1+length(sym_prefix), length($3)-length(sym_prefix))
}

# Process 'dumpbin /SYMBOLS <c_source.obj>' output.
#
# Handle lines like:
#   353 00008098 SECT4  notype       External     | opt_junk
#   3F1 00000000 SECT7  notype ()    External     | malloc_initialized
($3 ~ /^SECT[0-9]+/ && $(NF-2) == "External" && !($NF in exported_symbols)) {
  print $NF
}
