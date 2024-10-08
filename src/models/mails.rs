fn format_duration(seconds: u64) -> String {
    if seconds >= 3600 {
        let hours = seconds / 3600;
        format!("{} hour{}", hours, if hours > 1 { "s" } else { "" })
    } else if seconds >= 60 {
        let minutes = seconds / 60;
        format!("{} minute{}", minutes, if minutes > 1 { "s" } else { "" })
    } else {
        format!("{} second{}", seconds, if seconds > 1 { "s" } else { "" })
    }
}

pub fn generate_otp_email(code: &str, email: &str, otp_ttl: u64) -> String {
    format!(r#"
        <!DOCTYPE html>
        <html lang="en">
           <head>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
              <title>Your Bookeat OTP code</title>
           </head>
           <body>
              <div
                 style='background-color:#FFFFFF;color:#262626;font-family:"Helvetica Neue", "Arial Nova", "Nimbus Sans", Arial, sans-serif;font-size:16px;font-weight:400;letter-spacing:0.15008px;line-height:1.5;margin:0;padding:32px 0;min-height:100%;width:100%'
                 >
                 <table
                    align="center"
                    width="100%"
                    style="margin:0 auto;max-width:600px;background-color:#FFFFFF"
                    role="presentation"
                    cellspacing="0"
                    cellpadding="0"
                    border="0"
                    >
                    <tbody>
                       <tr style="width:100%">
                          <td>
                             <div style="padding:4px 16px 4px 16px">
                                <div style="padding:4px 16px 4px 16px;text-align:center">
                                   <a
                                      href="https://www.bookeat.ch/"
                                      style="text-decoration:none"
                                      target="_blank"
                                      ><img
                                      alt="Bookeat Logo"
                                      src="https://www.bookeat.ch/web/image/website/1/logo/BookEat?unique=966c57e"
                                      width="150"
                                      style="width:150px;outline:none;border:none;text-decoration:none;vertical-align:middle;display:inline-block;max-width:100%"
                                      /></a>
                                </div>
                             </div>
                             <h3
                                style='font-weight:bold;text-align:center;margin:0;font-family:"Helvetica Neue", "Arial Nova", "Nimbus Sans", Arial, sans-serif;font-size:20px;padding:4px 16px 4px 16px'
                                >
                                Your OTP code for Bookeat
                             </h3>
                             <div
                                style="font-weight:normal;text-align:center;padding:4px 16px 4px 16px"
                                >
                                <p>
                                   Use this code to sign up to Bookeat<br />This code will expire
                                   in {}
                                </p>
                             </div>
                             <h2
                                style="font-weight:bold;text-align:center;margin:0;font-size:24px;padding:4px 16px 4px 16px;letter-spacing:16px;"
                                >{}</h2>
                             <div
                                style="font-weight:normal;text-align:center;padding:4px 16px 4px 16px"
                                >
                                <p>
                                   <strong>This code will securely sign you up using</strong
                                      ><br />
                                <p style="color:#2a14b5"
                                   >{}</p
                                   >
                                </p>
                             </div>
                             <div
                                style="color:#ababab;font-weight:normal;text-align:center;padding:4px 16px 4px 16px"
                                >
                                If you don&#x27;t request this email, you can safely ignore it.
                             </div>
                          </td>
                       </tr>
                    </tbody>
                 </table>
              </div>
           </body>
        </html>
    "#, format_duration(otp_ttl), code, email)
}

pub fn generate_magic_link_email(callback_url: &str, email: &str, link_ttl: u64) -> String {
    format!(r#"
        <!DOCTYPE html>
        <html lang="en">
           <head>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
              <title>Your Bookeat magic link</title>
           </head>
           <body>
              <div
                 style='background-color:#FFFFFF;color:#262626;font-family:"Helvetica Neue", "Arial Nova", "Nimbus Sans", Arial, sans-serif;font-size:16px;font-weight:400;letter-spacing:0.15008px;line-height:1.5;margin:0;padding:32px 0;min-height:100%;width:100%'
                 >
                 <table
                    align="center"
                    width="100%"
                    style="margin:0 auto;max-width:600px;background-color:#FFFFFF"
                    role="presentation"
                    cellspacing="0"
                    cellpadding="0"
                    border="0"
                    >
                    <tbody>
                       <tr style="width:100%">
                          <td>
                             <div style="padding:4px 16px 4px 16px">
                                <div style="padding:4px 16px 4px 16px;text-align:center">
                                   <a
                                      href="https://www.bookeat.ch/"
                                      style="text-decoration:none"
                                      target="_blank"
                                      ><img
                                      alt="Bookeat Logo"
                                      src="https://www.bookeat.ch/web/image/website/1/logo/BookEat?unique=966c57e"
                                      width="150"
                                      style="width:150px;outline:none;border:none;text-decoration:none;vertical-align:middle;display:inline-block;max-width:100%"
                                      /></a>
                                </div>
                             </div>
                             <h3
                                style='font-weight:bold;text-align:center;margin:0;font-family:"Helvetica Neue", "Arial Nova", "Nimbus Sans", Arial, sans-serif;font-size:20px;padding:4px 16px 4px 16px'
                                >
                                Your magic link for Bookeat
                             </h3>
                             <div
                                style="font-weight:normal;text-align:center;padding:4px 16px 4px 16px"
                                >
                                <p>
                                   Use this link to sign up to Bookeat<br />This link will expire
                                   in {}
                                </p>
                             </div>
                             <div style="text-align: center;">
                                <a
                                   href="{}"
                                   style="color:#ffffff;font-size:16px;font-weight:normal;background-color:#241ab1;border-radius:8px;display:inline-block;padding:8px 12px;text-decoration:none;margin:16px auto;text-align:center;"
                                   target="_blank"
                                   >
                                   <span>Sign In</span
                                      ><span
                                          ><!--[if mso]><i
                                              style="letter-spacing: 12px;mso-font-width:-100%"
                                              hidden
                                              >&nbsp;</i
                                          ><!
                                          [endif]--></span
                                      >
                                   </a>
                             </div>
                             <div
                                style="font-weight:normal;text-align:center;padding:4px 16px 4px 16px"
                                >
                                <p>
                                   <strong>This link will securely sign you up using</strong
                                      ><br />
                                <p style="color:#2a14b5"
                                   >{}</p
                                   >
                                </p>
                             </div>
                             <div
                                style="color:#ababab;font-weight:normal;text-align:center;padding:4px 16px 4px 16px"
                                >
                                If you don&#x27;t request this email, you can safely ignore it.
                             </div>
                          </td>
                       </tr>
                    </tbody>
                 </table>
              </div>
           </body>
        </html>
    "#, format_duration(link_ttl), callback_url, email)
}