syntax on
filetype plugin indent on
filetype indent on

set spelllang=en
set nospell
set linebreak
set tabstop=2
set number relativenumber
set expandtab
set shiftround
set softtabstop=2
set shiftwidth=2
set autoindent
set smarttab
set tags=tags
set completeopt-=preview
set re=0
set backspace=indent,eol,start

call plug#begin()
  Plug 'preservim/nerdtree' " Side bar listing files
  Plug 'tpope/vim-surround' " Surround text with delimiters
  Plug 'tpope/vim-commentary' " Comment out lines
  Plug 'junegunn/fzf.vim' " Fuzzy finder
  Plug 'junegunn/fzf' " Fuzzy finderj
  Plug 'github/copilot.vim' " AI code completion
  Plug 'vim-airline/vim-airline' " Nice status bar
  Plug 'Quramy/tsuquyomi' " Typescript Server
  Plug 'pangloss/vim-javascript'  " Javascript support
  Plug 'leafgarland/typescript-vim' " Typescript support
  Plug 'neoclide/coc.nvim', {'branch': 'release'} " Code completion
  Plug 'psliwka/vim-smoothie' " Smooth vim scrolling
  Plug 'peitalin/vim-jsx-typescript' " Nice jsx syntax
  Plug 'tpope/vim-fugitive' " Git integration
call plug#end()

" Easy Escape
imap jj <Esc>

" Nerd tree 
nnoremap <C-n> :NERDTree<CR>
" Find current file in nerdtree
nmap ,n :NERDTreeFind<CR>

" fzf search all files names not in gitignore
nnoremap <C-p> :GFiles --cached --others --exclude-standard<CR>
" fzf search all files content
nnoremap <C-f> :Rg<CR>

" Enter to accept coc option
inoremap <expr> <cr> coc#pum#visible() ? coc#pum#confirm() : "\<CR>"


" coc goto code navigation
nmap <silent> gd <Plug>(coc-definition)
nmap <silent> gD :call CocAction('jumpDefinition', 'vsplit')<CR>
nmap <silent> gy <Plug>(coc-type-definition)
nmap <silent> gi <Plug>(coc-implementation)
nmap <silent> gr <Plug>(coc-references)

" Run the Code Lens action on the current line.
nmap <leader>cl  <Plug>(coc-codelens-action)

" Tooltips on hover
nnoremap <silent> <leader>h :call CocActionAsync('doHover')<cr>

" Highlight the symbol and its references when holding the cursor.
autocmd CursorHold * silent call CocActionAsync('highlight')


