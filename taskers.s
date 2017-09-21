let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd ~/programming/rust/taskers
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +16 src/lib.rs
badd +1 src/main.rs
badd +25 ~/vimwiki/index.wiki
badd +1 ~/vimwiki/Taskers.wiki
argglobal
silent! argdel *
argadd src/lib.rs
argadd src/main.rs
edit src/main.rs
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd _ | wincmd |
split
1wincmd k
wincmd w
wincmd w
set nosplitbelow
set nosplitright
wincmd t
set winminheight=1 winminwidth=1 winheight=1 winwidth=1
exe '1resize ' . ((&lines * 24 + 26) / 53)
exe 'vert 1resize ' . ((&columns * 78 + 79) / 159)
exe '2resize ' . ((&lines * 26 + 26) / 53)
exe 'vert 2resize ' . ((&columns * 78 + 79) / 159)
exe 'vert 3resize ' . ((&columns * 80 + 79) / 159)
argglobal
edit src/main.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 22 - ((15 * winheight(0) + 12) / 24)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
22
normal! 031|
wincmd w
argglobal
edit ~/vimwiki/Taskers.wiki
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 19 - ((12 * winheight(0) + 13) / 26)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
19
normal! 038|
wincmd w
argglobal
edit src/lib.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 39 - ((25 * winheight(0) + 25) / 51)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
39
normal! 025|
wincmd w
3wincmd w
exe '1resize ' . ((&lines * 24 + 26) / 53)
exe 'vert 1resize ' . ((&columns * 78 + 79) / 159)
exe '2resize ' . ((&lines * 26 + 26) / 53)
exe 'vert 2resize ' . ((&columns * 78 + 79) / 159)
exe 'vert 3resize ' . ((&columns * 80 + 79) / 159)
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToO
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
