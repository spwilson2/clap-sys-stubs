from ast import parse
import re
import os
import struct 
import textwrap

# https://www.w3resource.com/python-exercises/string/python-data-type-string-exercise-96.php
def camel_case(s):
  s = re.sub(r"(_|-)+", " ", s).title().replace(" ", "")
  return ''.join([s[0].lower(), s[1:]])

def gen_source(writer, struct_name, data_fields, fn_fields):

    initializers = []
    # Write function stubs, save initializers
    for ident, typesig in fn_fields.items():
        writer.write(textwrap.dedent(f'''\
            unsafe extern "C" fn {ident}{typesig} {{todo!()}}
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
    }}
    impl {trait_name} for {struct_name} {{
        fn new() -> Self {{
            Self {{
{initializers}
            }}
        }}
    }}
    '''))

    


def parse_gen_file(src_file, dst_file):
    fwriter = None
    with open(src_file, 'r') as f:
        # Find each struct,
        for m in re.finditer(r'struct (?P<ident>[\w_]*) {(?P<contents>.*?)}', f.read(), re.DOTALL):
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

            if fn_fields or data_fields:
                if not fwriter:
                    fwriter = open(dst_file, 'w')
                gen_source(fwriter, struct_name, data_fields, fn_fields)

        #for l in f:
        #    ## Iterate through, look for struct <ident>
        #    #if m := re.search('struct (?P<ident>[\w_]*)', l):
        #    #    struct_name = m.group('ident')

        #    if m := re.search('pub (?P<ident>[\w_]*): (?P<type>.*),', l):
        #        # TODO Now check if the field is a func ptr
        #        ident = m['ident']
        #        if m:= re.search(r'Option\<unsafe extern "C" fn(?P<type>\(.*)\>', m['type']):
        #            struct_fields[ident] = m['type']

        #    #terminate at } 
        #    if m := re.search('}', l):
        #        if struct_fields:
        #            if not fwriter:
        #                fwriter = open(dst_file, 'w')
        #            gen_source(fwriter, struct_name, struct_fields)
        #        struct_name = None
        #        struct_fields = {}
    if fwriter:
        fwriter.close()

for (dirpath, dirnames, filenames) in os.walk('clap-sys/src'):
    for f in filenames:
        (_, ext) = os.path.splitext(f)
        if ext != '.rs':
            continue

        dst_fname = f

        # Don't ovewrite lib.rs since we'll keep manually written imports there.
        if dst_fname == 'lib.rs':
            dst_fname = 'lib_gen.rs'
        if dst_fname == 'mod.rs':
            dst_fname = 'mod_gen.rs'

        # Trim out clap-sys
        dst_dirpath = dirpath[len('clap-sys/'):]
        if not os.path.exists(dst_dirpath):
            os.makedirs(dst_dirpath)
        dst_file = os.path.join(dst_dirpath, dst_fname)
        src_path = os.path.join(dirpath, f)
        parse_gen_file(src_path, dst_file)

