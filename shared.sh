readf () {
  file="$(greadlink -f "$1")"
  cat "$file"
}

replace () {
  sed "s/{{$1}}/$2/g"
}

copy () {
  echo "🚚  $1 -> $2" >&2
  cp "$1" "$2"
}

to_camel_case () {
  sed 's/[_|-]\([a-z]\)/\U\1/g;s/^\([a-z]\)/\1/g'
}
