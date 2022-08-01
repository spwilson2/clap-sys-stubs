from ast import parse
import re
import os
import struct 
import textwrap
from pathlib import Path
import subprocess

# Adjusted from 
# https://www.w3resource.com/python-exercises/string/python-data-type-string-exercise-96.php
def camel_case(s):
  s = re.sub(r"(_|-)+", " ", s).title().replace(" ", "")
  return ''.join(s)

def gen_source_for_struct(writer, struct_name, data_fields, fn_fields):

    initializers = []
    # Write function stubs, save initializers
    for ident, typesig in fn_fields.items():
        writer.write(textwrap.dedent(f'''\
            unsafe extern "C" fn {ident}{typesig} {{todo!()}}\n
        '''))

        initializers.append(f'{ident}: Some({ident})')

    for ident in data_fields:
        initializers.append(f'{ident}: todo!()')

    initializers = ',\n'.join(initializers)

    # Generate code to initialize the struct with pointers
    # E.g.
    # trait ClapHostInit { fn new() -> Self; }
    # trait ClapHostInit {
    #     fn new() -> Self {
    #         Self {
    #             clap_version: todo!(),
    #             get_extension: Some(get_extension),
    #         }
    #     }
    # }
    trait_name = camel_case(struct_name) + "Init"
    writer.write(textwrap.dedent(f'''\
    pub trait {trait_name} {{
        fn new() -> Self;
    }}\n
    impl {trait_name} for {struct_name} {{
        fn new() -> Self {{
            Self {{
{initializers}
            }}
        }}
    }}\n
    '''))

    

def get_module_name_from_filepath(crate, src_file):
    src_mod_name = os.path.splitext(os.path.basename(src_file))[0]

    paths = Path(src_file).parts
    # Get to src
    src_idx = paths.index('src')
    # Then all names after that are module name
    mods = [crate] + list(paths[src_idx+1:len(paths)-1]) + [src_mod_name]
    return '::'.join(mods)

def gen_import_statements(writer, src_name, existing_imports):
    crate = 'clap_sys'
    clap_sys_mod = get_module_name_from_filepath(crate, src_name)
    writer.write(f'use {clap_sys_mod}::*;\n')
    for i in existing_imports:
        i = re.sub(' crate::', f' {crate}::', i)
        # Translate any crate:: into ${crate}
        writer.write(i + '\n')

def gen_mod_statements(writer, mod_statements):
    for m in mod_statements:
        writer.write(m + '\n')

def gen_warning_supresses(writer):
    writer.write('#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]\n')

def parse_gen_file(src_file, dst_file):
    with open(src_file, 'r') as freader, open(dst_file, 'w') as fwriter:
        # Write supression for any warnings stub methods will cause
        gen_warning_supresses(fwriter)

        # Find all use statements
        mod_statements = []
        for m in re.finditer(r'pub mod .*?;', freader.read(), re.DOTALL):
            mod_statements.append(m.group(0))
        gen_mod_statements(fwriter, mod_statements)

        # # Skip writing use statements for mod crates, they only export heirarchy.
        if not mod_statements:
            # Reparse file for use statements
            freader.seek(0)
            use_statements = []
            for m in re.finditer(r'use .*?;', freader.read(), re.DOTALL):
                use_statements.append(m.group(0))
            gen_import_statements(fwriter, src_file, use_statements)

        # Split up the use from the impls by a line.
        fwriter.write('\n')

        # Reparse file for structs
        freader.seek(0)
        for m in re.finditer(r'struct (?P<ident>[\w_]*) {(?P<contents>.*?)}', freader.read(), re.DOTALL):
            struct_name = m['ident']
            print(struct_name)

            # Find all fn fields
            struct_contents = m['contents']
            fn_fields = {} 
            data_fields = {}

            #fn field regex
            for m in re.finditer(r'pub\s+(?P<ident>[\w_]*)\s*:\s+Option\s*<(?P<t>.*?)[,\s]*>,', struct_contents, re.DOTALL):
                ident = m['ident']
                # reduce whitespace
                t = re.sub('\s+', ' ', m['t']).rstrip().lstrip()
                # Now just grab the param and return type
                if m:= re.search(r'unsafe extern "C" fn(?P<type>\(.*)', t):
                    fn_fields[ident] = m['type']

            #data field regex, start broad, then verify it's not a callback
            for m in re.finditer(r'pub\s+(?P<ident>[\w_]*)\s*:(?P<type>.*?),', struct_contents, re.DOTALL):
                ident = m['ident']
                type_ = m['type']
                if re.search(r'fn\s*\(', type_):
                    continue
                type_ = type_.rstrip().lstrip()
                data_fields[ident] = type_

            gen_source_for_struct(fwriter, struct_name, data_fields, fn_fields)

if __name__ == '__main__':
    # Enable running from anywhere
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    for (dirpath, dirnames, filenames) in os.walk('clap-sys/src'):
        for f in filenames:
            (_, ext) = os.path.splitext(f)
            if ext != '.rs':
                continue

            # Trim out clap-sys path, write to our own src
            dst_dirpath = dirpath[len('clap-sys/'):]
            if not os.path.exists(dst_dirpath):
                os.makedirs(dst_dirpath)
            dst_file = os.path.join(dst_dirpath, f)
            src_path = os.path.join(dirpath, f)
            parse_gen_file(src_path, dst_file)

    subprocess.check_call('cargo fmt', shell=True)
    subprocess.check_call('cargo check', shell=True)