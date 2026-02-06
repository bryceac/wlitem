# WlItem

**Author:** Bryce Campbell

**License:** See LICENSE

**Description:** a crate for Rust that provides a model that represents wishlist items.

**Version:** 0.1.1

## Notes

This crate has been tested on macOS 15.7.3 under Rust 1.88.0. It is not known if it will work on earlier versions or on other platforms, but there **should not** be any issues as long as the version of rust being used 
is the 2024 edition or newer.

### Why Create this library?

While there are many solutions to create a wishlist, many of which are online, or one could just whip up an HTML file or Markdown file, I wanted to create an application that did not use the Internet 
that I could use in place of my current setup, which involved: 
 
  1. making a Word document with links
  2. export Word document to HTML
  3. clean up HTML and add notes to items

Although this crate does not facilitate the creation of HTML pages, it does house the building block necessary for what I want, which are as follows:

  * allow items to have multiple notes
  * specify the priority level of desired item
  * provide enough details to turn item into a link

As of the writing of this README, each of those has been implemented.

### Version History

<dl>
  <dt style="font-weight:bold">0.1</dt>
  <dd>Initial release. Released Feb. 3, 2026</dd>
  <dt style="font-weight:bold">0.1.1</dt>
  <dd>Minor fixes to allow easier utilization with Clap. Released Feb. 5, 2026</dd>
</dl>

### Usage

To use this library, add the following to your **Cargo.toml** file:

<pre>
[dependency]
wlitem = "0.1"
</pre>

Afterwards, you can do something as simple as this, if you just need the Item type:

<pre>
use wlitem::Item;
</pre>

There are other types included in this crate, but most are not going to be useful, beyond those that are as easily importable as the **Item** type.

#### Creating Items

To create an item, it is recommended to use the **builder** function as it others the best flexibility, and it is also utilized in the other available options.

Using the builder would look like this:

<pre>
  use wlitem::Item;

  let its_a_wonderful_life = Item::builder()
  .set_name("It's a Wonderful Life")
  .set_quantity(1)
  .set_priority("high")
  .set_url("https://example.com/its_a_wonderful_life")
  .add_note("4K Blu Ray")
  .build();
</pre>

If you just want an empty item, you can do this:

<pre>
  Item::new();
</pre>

This is the exact same as doing this:

<pre>
  Item::builder()
  .build();
</pre>

IDs are set using the **set_id** method on the builder.

When specifying a URL, it **must** be a full and valid URL or no URL will be attributed to the URL field.

For sample code purposes, you can also use this, but it is not recommended for use outside of creating sample items:

<pre>
  let nintendo_switch_2 = Item::from("7746C39C-D951-4E03-840C-8E91AF0B6D1D", "Nintendo Switch 2", 1, "high", "https://example.com/nintendo_switch_2", vec![]);
</pre>

This will create an item with any notes, but specifies all other fields.

#### Saving Data

To save data, the only thing that should be necessary is to import the **Save** trait and the **Item** type, which will enable a save method on vectors that contain values of the latter type

The code would look like this:

<pre>
  use wlitem::{ Item, Save };

  let items: Vec<Item> = vec![];

  if let Err(error) = items.save("wishlist.json") {
      println!("{}", error)
  }
</pre>

This will save the items to a JSON file.

If you instead want TSV output, use **save.tsv** instead.

The only difference between them is that only the JSON output will include any notes to attached to the items. This decision was made because notes can be complicated, since I cannot predict if notes will be single lined or not 
and my own wishlists have had multiple lines at one point.

#### Loading Data

To load data, it is only necessary to import the **Item** type, and use the appropriate method.

The code will ultimately look like this:

<pre>
  use wlitem::Item;

  let Ok(parsed_items) = Item::from_file("wishlist.json") {
    println!("{}", parsed_items[0].name);
  }
</pre>

This will load items from a JSON file. If you want to load tsv data instead, you would use **from_tsv_file**.

Please be aware that TSV imports do not support column headings, and the TSV will be interpreted like this:

  1. id
  2. name
  3. quantity
  4. priority
  5. url

All fields are optional value wise, but the TSV file must contain all five fields.
The id will automatically set unless given a value, while quantity defaults to 1 and priority defaults to medium.
The URL is optional because the item might not necessarily be found online or is generic enough to find easily, thereby it will be empty is none is provided.

### Contributing

If you think you can help make this crate better, feel free to fork this project and make a pull request.

I will test the changes to see if they work as they should.

### Support

Although I am using Rust a lot these days, I still feel that my experience is very limited, so I cannot provide much support.

As such, expect to be on your own, but I am willing to try and check things out if you contact me at the email below:

tonyhawk2100@gmail.com
