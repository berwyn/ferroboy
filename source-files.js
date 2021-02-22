var N = null;var sourcesIndex = {};
sourcesIndex["addr2line"] = {"name":"","files":["lazy.rs","lib.rs"]};
sourcesIndex["adler"] = {"name":"","files":["algo.rs","lib.rs"]};
sourcesIndex["ansi_term"] = {"name":"","files":["ansi.rs","debug.rs","difference.rs","display.rs","lib.rs","style.rs","util.rs","windows.rs","write.rs"]};
sourcesIndex["backtrace"] = {"name":"","dirs":[{"name":"backtrace","files":["libunwind.rs","mod.rs"]},{"name":"symbolize","dirs":[{"name":"gimli","files":["elf.rs","mmap_unix.rs","stash.rs"]}],"files":["gimli.rs","mod.rs"]}],"files":["capture.rs","lib.rs","print.rs","types.rs"]};
sourcesIndex["bitflags"] = {"name":"","files":["lib.rs"]};
sourcesIndex["cfg_if"] = {"name":"","files":["lib.rs"]};
sourcesIndex["color_eyre"] = {"name":"","dirs":[{"name":"section","files":["help.rs","mod.rs"]}],"files":["config.rs","handler.rs","lib.rs","private.rs","writers.rs"]};
sourcesIndex["color_spantrace"] = {"name":"","files":["lib.rs"]};
sourcesIndex["eyre"] = {"name":"","files":["backtrace.rs","chain.rs","context.rs","error.rs","fmt.rs","kind.rs","lib.rs","macros.rs","wrapper.rs"]};
sourcesIndex["ferroboy"] = {"name":"","dirs":[{"name":"assembly","files":["disassemble.rs","instruction.rs","instruction_stream.rs","mod.rs"]},{"name":"operations","dirs":[{"name":"add","files":["mod.rs","narrow.rs","wide.rs"]},{"name":"dec","files":["mod.rs","narrow.rs","wide.rs"]},{"name":"inc","files":["mod.rs","narrow.rs","wide.rs"]},{"name":"jump","files":["mod.rs","position.rs","relative.rs"]},{"name":"load","files":["mod.rs","narrow.rs","wide.rs"]},{"name":"rotate","files":["left_carry.rs","mod.rs","right_carry.rs"]}],"files":["and.rs","call.rs","cp.rs","halt.rs","interrupts.rs","mod.rs","nop.rs","operation.rs","or.rs","pop.rs","prefix.rs","push.rs","ret.rs","rst.rs","stop.rs","sub.rs","xor.rs"]},{"name":"system","dirs":[{"name":"register","files":["mod.rs","narrow.rs","wide.rs"]}],"files":["alu.rs","cartridge.rs","config.rs","cpu.rs","mmu.rs","mod.rs","opcodes.rs"]}],"files":["error.rs","helpers.rs","lib.rs","state.rs"]};
sourcesIndex["ferroboy_core"] = {"name":"","files":["ferroboy_core.rs","lib.rs"]};
sourcesIndex["ferroboy_dasm"] = {"name":"","files":["main.rs"]};
sourcesIndex["gimli"] = {"name":"","dirs":[{"name":"read","files":["abbrev.rs","addr.rs","aranges.rs","cfi.rs","dwarf.rs","endian_slice.rs","line.rs","loclists.rs","lookup.rs","mod.rs","op.rs","pubnames.rs","pubtypes.rs","reader.rs","rnglists.rs","str.rs","unit.rs","value.rs"]}],"files":["arch.rs","common.rs","constants.rs","endianity.rs","leb128.rs","lib.rs"]};
sourcesIndex["indenter"] = {"name":"","files":["lib.rs"]};
sourcesIndex["jane_eyre"] = {"name":"","files":["lib.rs"]};
sourcesIndex["lazy_static"] = {"name":"","files":["inline_lazy.rs","lib.rs"]};
sourcesIndex["libc"] = {"name":"","dirs":[{"name":"unix","dirs":[{"name":"linux_like","dirs":[{"name":"linux","dirs":[{"name":"gnu","dirs":[{"name":"b64","dirs":[{"name":"x86_64","files":["align.rs","mod.rs","not_x32.rs"]}],"files":["mod.rs"]}],"files":["align.rs","mod.rs"]}],"files":["align.rs","mod.rs"]}],"files":["mod.rs"]}],"files":["align.rs","mod.rs"]}],"files":["fixed_width_ints.rs","lib.rs","macros.rs"]};
sourcesIndex["libretro_backend"] = {"name":"","files":["lib.rs"]};
sourcesIndex["libretro_sys"] = {"name":"","files":["lib.rs"]};
sourcesIndex["miniz_oxide"] = {"name":"","dirs":[{"name":"deflate","files":["buffer.rs","core.rs","mod.rs","stream.rs"]},{"name":"inflate","files":["core.rs","mod.rs","output_buffer.rs","stream.rs"]}],"files":["lib.rs","shared.rs"]};
sourcesIndex["object"] = {"name":"","dirs":[{"name":"read","dirs":[{"name":"coff","files":["file.rs","mod.rs","relocation.rs","section.rs","symbol.rs"]},{"name":"elf","files":["compression.rs","file.rs","mod.rs","note.rs","relocation.rs","section.rs","segment.rs","symbol.rs"]},{"name":"macho","files":["file.rs","load_command.rs","mod.rs","relocation.rs","section.rs","segment.rs","symbol.rs"]},{"name":"pe","files":["file.rs","mod.rs","section.rs"]}],"files":["any.rs","mod.rs","traits.rs","util.rs"]}],"files":["common.rs","elf.rs","endian.rs","lib.rs","macho.rs","pe.rs","pod.rs"]};
sourcesIndex["once_cell"] = {"name":"","files":["imp_std.rs","lib.rs","race.rs"]};
sourcesIndex["pico_args"] = {"name":"","files":["lib.rs"]};
sourcesIndex["proc_macro2"] = {"name":"","files":["detection.rs","fallback.rs","lib.rs","marker.rs","parse.rs","wrapper.rs"]};
sourcesIndex["quote"] = {"name":"","files":["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]};
sourcesIndex["rustc_demangle"] = {"name":"","files":["legacy.rs","lib.rs","v0.rs"]};
sourcesIndex["sharded_slab"] = {"name":"","dirs":[{"name":"page","files":["mod.rs","slot.rs","stack.rs"]},{"name":"pool","files":["mod.rs"]}],"files":["cfg.rs","clear.rs","implementation.rs","iter.rs","lib.rs","shard.rs","sync.rs","tid.rs"]};
sourcesIndex["syn"] = {"name":"","dirs":[{"name":"gen","files":["clone.rs","debug.rs","eq.rs","gen_helper.rs","hash.rs","visit_mut.rs"]}],"files":["attr.rs","await.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","reserved.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","tt.rs","ty.rs","verbatim.rs","whitespace.rs"]};
sourcesIndex["thiserror"] = {"name":"","files":["aserror.rs","display.rs","lib.rs"]};
sourcesIndex["thiserror_impl"] = {"name":"","files":["ast.rs","attr.rs","expand.rs","fmt.rs","lib.rs","prop.rs","valid.rs"]};
sourcesIndex["thread_local"] = {"name":"","files":["cached.rs","lib.rs","thread_id.rs","unreachable.rs"]};
sourcesIndex["tracing"] = {"name":"","files":["dispatcher.rs","field.rs","level_filters.rs","lib.rs","macros.rs","span.rs","stdlib.rs","subscriber.rs"]};
sourcesIndex["tracing_attributes"] = {"name":"","files":["lib.rs"]};
sourcesIndex["tracing_core"] = {"name":"","files":["callsite.rs","dispatcher.rs","event.rs","field.rs","lib.rs","metadata.rs","parent.rs","span.rs","stdlib.rs","subscriber.rs"]};
sourcesIndex["tracing_error"] = {"name":"","files":["backtrace.rs","error.rs","layer.rs","lib.rs"]};
sourcesIndex["tracing_subscriber"] = {"name":"","dirs":[{"name":"field","files":["debug.rs","delimited.rs","display.rs","mod.rs"]},{"name":"filter","files":["level.rs","mod.rs"]},{"name":"fmt","dirs":[{"name":"format","files":["mod.rs"]},{"name":"time","files":["datetime.rs","mod.rs"]}],"files":["fmt_layer.rs","mod.rs","writer.rs"]},{"name":"registry","files":["extensions.rs","mod.rs","sharded.rs","stack.rs"]}],"files":["layer.rs","lib.rs","prelude.rs","reload.rs","sync.rs","thread.rs","util.rs"]};
sourcesIndex["unicode_xid"] = {"name":"","files":["lib.rs","tables.rs"]};
createSourceSidebar();
