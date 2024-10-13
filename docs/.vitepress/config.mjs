import footnote from 'markdown-it-footnote'
import { withMermaid } from "vitepress-plugin-mermaid"

// https://vitepress.dev/reference/site-config
export default withMermaid({
  title: "Warp Zcash Wallet Library",
  description: "A High Performance Wallet Synchronization and Transaction Builder for Zcash",
  base: '/warp/',
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
    ],

    sidebar: [
      {
        text: 'About Zcash',
        collapsible: true,
        collapsed: true,
        items: [
          { text: 'What is Zcash?', link: '/about-zcash/' },
          { text: 'What can you use it for?', link: '/about-zcash/usage' },
        ]
      },
      {
        text: 'Tutorial',
        collapsible: true,
        collapsed: true,
        items: [
          { text: 'Overview', link: '/tutorial/' },
          { text: 'Prerequisites', link: '/tutorial/prereq' },
          { text: 'Setup Zcashd Regtest', link: '/tutorial/zcashd' },
          { text: 'Setup Zcash-Warp Regtest', link: '/tutorial/warp' },
          { text: 'Create a new account', link: '/tutorial/account' },
          { text: 'Mining', link: '/tutorial/mine' },
          { text: 'Setup Lightwalletd', link: '/tutorial/lwd' },
          { text: 'Shield Coinbase Reward', link: '/tutorial/shieldcoinbase' },
          { text: 'Synchronization', link: '/tutorial/sync' },
          { text: 'Basic Transaction & Fees', link: '/tutorial/simple_tx' },
          { text: 'Splitting Notes', link: '/tutorial/split' },
          { text: 'Diverse Input Notes', link: '/tutorial/multi_notes' },
          { text: 'Paying a Transparent Address', link: '/tutorial/pay_t' },
          { text: 'Paying a Shielded Address', link: '/tutorial/pay_z' },
          { text: 'Paying a Multi-Receiver UA', link: '/tutorial/pay_ua' },
          { text: 'Using Payment URIs', link: '/tutorial/payment_uri' },
          { text: 'Max transfer', link: '/tutorial/max' },
          { text: 'Conclusion', link: '/tutorial/conclusion' },
        ]
      },
      {
        text: 'ZKool',
        collapsible: true,
        collapsed: true,
        items: [
          { text: 'News', link: '/zkool/' },
          { text: 'Accounts', link: '/zkool/accounts' },
        ]
      },
      {
        text: 'Coin Voting',
        collapsible: true,
        collapsed: true,
        items: [
          { text: 'Overview', link: '/coin_voting/' },
          { text: 'Mindmap', link: '/coin_voting/mindmap' },
          { text: 'User Stories', link: '/coin_voting/user_stories' },
        ]
      },
      {
        text: 'Expert',
        collapsible: true,
        collapsed: true,
        items: [
          { text: 'Overview', link: '/expert/' },
          { text: 'Coin Voting', link: '/expert/coinvoting' },
          { text: 'Mindmap', link: '/expert/mindmap' },
        ]
      },
      {
        text: 'Test Cases',
        collapsible: true,
        collapsed: true,
        items: [
          { text: 'Setup', link: '/tests/' },
          { text: 'Accounts', link: '/tests/accounts/' },
          { text: 'Test Areas', link: '/tests/areas/' },
        ]
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/hhanh00/warp' }
    ]
  },
  markdown: {
    config: (md) => {
      md.use(footnote)
    }
  }
})
