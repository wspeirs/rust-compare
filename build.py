from jinja2 import Environment, FileSystemLoader
from side_menu import MENU_ITEMS
import re


def read_code(file_name):
    ret = []
    add_line = False
    ws_count = -1

    with open(file_name, 'r') as f:
        for line in f:
            if '// END_CODE' in line:
                break

            if add_line:
                line = line.replace(r' {3,8}', '\t')
                line = line.replace('\t', '  ')

                if ws_count == -1:
                    ws_count = re.search(r'[\S]', line).start()
                ret.append(line[ws_count:] if len(line) >= ws_count else line)

            if '// BEGIN_CODE' in line:
                add_line = True

    if len(ret) == 0:
        print "Error did not read any code from %s. Make sure you have a BEGIN_CODE marker" % file_name

    return ret


env = Environment(loader=FileSystemLoader('templates'))
env.autoescape = True

# go through and "fix-up" MENU_ITEMS
for item in list(MENU_ITEMS):
    if 'name' not in item:
        print "Error couldn't find name in item: %s" % str(item)
        MENU_ITEMS.remove(item)

    if 'page' not in item:
        print "Error, no page found in item: %s" % str(item)
        MENU_ITEMS.remove(item)
        continue

    if item['page'] is None:
        continue

    page = item['page']

    if 'template' not in page:
        print "Warning, couldn't find template for %s skipping..." % item['name']
        page['template'] = 'not_found.html'
        continue

# make the not-found page
template = env.get_template('not_found.html')

with open('site/not_found.html', 'w') as f:
    f.writelines(template.render(page_title='Not Found', menu_items=MENU_ITEMS))

for item in MENU_ITEMS:
    if item['page'] is None:
        continue

    page = item['page']

    if page['template'] == 'not_found.html':
        continue

    template = env.get_template(page['template'])

    if page['cpp'] is None or \
       page['java'] is None or \
       page['rust'] is None:
        cpp_code = ''
        java_code = ''
        rust_code = ''
    else:
        cpp_code = read_code('code/cpp/' + page['cpp'])
        java_code = read_code('code/java/' + page['java'])
        rust_code = read_code('code/rust/' + page['rust'])

    with open('site/' + page['template'], 'w') as f:
        f.writelines(template.render(page_title=item['name'],
                                     menu_items=MENU_ITEMS,
                                     cpp_code=cpp_code,
                                     java_code=java_code,
                                     rust_code=rust_code))

        print "Created: %s\tcpp: %d  java: %d  rust: %d" % (page['template'], len(cpp_code), len(java_code), len(rust_code))

