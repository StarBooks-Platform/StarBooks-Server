using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using StarBooks.Domain.Core.Converters;

namespace StarBooks.Domain.Books;

[Table("categories"), JsonConverter(typeof(CategoryConverter))]
public class CategoryModel
{
   [Key]
   public int CategoryId { get; set; }
   
   public string Topic { get; set; }
   
   public ICollection<BookModel> Books { get; set; }

   public override string ToString()
   {
      return Topic;
   }
   
   public static CategoryModel Parse(string topic)
   {
      return new CategoryModel { Topic = topic };
   }
}