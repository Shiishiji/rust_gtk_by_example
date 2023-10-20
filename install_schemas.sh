#!/usr/bin/env sh

echo "Creating schemas directory.."
mkdir -p $HOME/.local/share/glib-2.0/schemas

echo "Coping files.."
cp com.shiishiji.my-gtk.app.Window.gschema.xml $HOME/.local/share/glib-2.0/schemas/
cp com.shiishiji.my-gtk.app.Settings1.gschema.xml $HOME/.local/share/glib-2.0/schemas/

echo "Compiling..."
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/

echo "Done"