# IPCCodegen.cmake - protoc-style wrapper for the ipc_codegen host tool.
#
# The ipc_codegen target itself is defined once in tools/CMakeLists.txt; consumers
# pull it in with add_subdirectory() and then invoke it through this helper, the
# same way protobuf_generate invokes the protoc target.

# opensteamtool_add_ipc_codegen(<out_var> IDL <file.steamd> CPP_OUT <dir>)
#
# Generates <CPP_OUT>/<idl-stem>.gen.h from the IDL and returns the generated
# header path in <out_var>, suitable for listing as a target source.
function(opensteamtool_add_ipc_codegen out_var)
    set(one_value_args IDL CPP_OUT)
    cmake_parse_arguments(IPC "" "${one_value_args}" "" ${ARGN})

    foreach(_required IDL CPP_OUT)
        if(NOT IPC_${_required})
            message(FATAL_ERROR "opensteamtool_add_ipc_codegen requires ${_required}")
        endif()
    endforeach()

    if(NOT TARGET ipc_codegen)
        message(FATAL_ERROR
            "opensteamtool_add_ipc_codegen: target 'ipc_codegen' is not defined; "
            "add_subdirectory() the tools directory before calling this function")
    endif()

    get_filename_component(_idl_name "${IPC_IDL}" NAME)
    get_filename_component(_idl_stem "${IPC_IDL}" NAME_WE)
    set(_header "${_idl_stem}.gen.h")
    set(_output "${IPC_CPP_OUT}/${_header}")

    add_custom_command(
        OUTPUT "${_output}"
        COMMAND ${CMAKE_COMMAND} -E make_directory "${IPC_CPP_OUT}"
        COMMAND $<TARGET_FILE:ipc_codegen> "--cpp_out=${IPC_CPP_OUT}" "${IPC_IDL}"
        DEPENDS "${IPC_IDL}" ipc_codegen
        COMMENT "Generating ${_header} from ${_idl_name}"
        VERBATIM
    )

    set(${out_var} "${_output}" PARENT_SCOPE)
endfunction()
