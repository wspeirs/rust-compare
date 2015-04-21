from flask import Flask, g, render_template, session, redirect, request, abort, Response
from side_menu import MENU_ITEMS

app = Flask(__name__)
app.debug = True


#
# Routes start here
#
@app.route('/')
def root():
    return render_template('index.html', menu_items=MENU_ITEMS)

#
# Run the app if called by Python
#
if __name__ == '__main__':
    app.run()
